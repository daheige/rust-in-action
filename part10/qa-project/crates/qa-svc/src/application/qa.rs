use crate::config::{AppState, APP_CONFIG};
use crate::domain::entity::{
    AnswersEntity, EntityReadCountData, QuestionsEntity, UserSessionEntity, VoteMessage,
};
use crate::domain::repository::{
    AnswerRepo, QuestionRepo, ReadCountRepo, UserRepo, UserSessionRepo, UserVoteRepo,
};
use crate::infrastructure::cache::new_user_cache;
use crate::infrastructure::persistence::{new_answer_repo, new_question_repo, new_user_repo};
use crate::infrastructure::read_count::new_read_count_repo;
use crate::infrastructure::vote::new_vote_repo;
use autometrics::autometrics;
use chrono::{Local, TimeZone};
use infras::{AesCBCCrypto, AesKeySize};
use log::info;
use pb::qa::qa_service_server::QaService;
use pb::qa::*;
use sqlx::sqlx_macros::expand_query;
use sqlx::types::chrono::NaiveDateTime;
use sqlx::FromRow;
use std::ops::Add;
use std::process::exit;
use tonic::{Code, Request, Response, Status};
use tracing_subscriber::fmt::format;
use uuid::Uuid;

/// 实现qa.proto 接口服务
struct QAServiceImpl {
    user_repo: Box<dyn UserRepo>,
    question_repo: Box<dyn QuestionRepo>,
    answer_repo: Box<dyn AnswerRepo>,
    read_count_repo: Box<dyn ReadCountRepo>,
    vote_repo: Box<dyn UserVoteRepo>,
    user_session_repo: Box<dyn UserSessionRepo>,
    aes_crypto: AesCBCCrypto,
}

// 创建QaService实例
pub fn new_qa_service(app_state: AppState) -> impl QaService {
    let user_repo = Box::new(new_user_repo(app_state.mysql_pool.clone()));
    let question_repo = Box::new(new_question_repo(app_state.mysql_pool.clone()));
    let answer_repo = Box::new(new_answer_repo(app_state.mysql_pool.clone()));
    let read_count_repo = Box::new(new_read_count_repo(
        app_state.redis_pool.clone(),
        app_state.mysql_pool.clone(),
    ));
    let user_cache_repo = Box::new(new_user_cache(app_state.redis_pool));
    let vote_repo = Box::new(new_vote_repo(app_state.mysql_pool, app_state.pulsar_client));
    QAServiceImpl {
        user_repo,
        user_session_repo: user_cache_repo,
        question_repo,
        answer_repo,
        read_count_repo,
        vote_repo,
        aes_crypto: AesCBCCrypto::new(&APP_CONFIG.aes_key, &APP_CONFIG.aes_iv, AesKeySize::Size256),
    }
}

impl QAServiceImpl {
    // 验证token是否有效
    // 返回登录时的token和expire_time过期时间
    fn check_token(&self, token: &str) -> Result<String, String> {
        if token.len() == 0 {
            return Err("token length invalid".to_string());
        }

        // 解密token
        let decrypted = self.aes_crypto.decrypt(token);
        if let Err(err) = decrypted {
            info!("failed to decrypt token:{},error:{:?}", token, err);
            return Err(format!("parse token error:{:?}", err));
        }
        let payload = decrypted.unwrap();
        let arr = payload.split(":").collect::<Vec<&str>>();
        let token = arr[0].to_string();
        if token.len() != 32 {
            return Err("token length invalid".to_string());
        }

        // 判断token是否过期
        let expired = arr[1].parse::<i64>();
        if let Err(err) = expired {
            return Err(format!("token expire_time parse error:{}", err));
        }
        let expire_time = expired.unwrap();
        let current_time = Local::now().timestamp();
        let elapsed = (current_time - expire_time).abs();
        if elapsed >= 86400 {
            return Err("token has expired".to_string());
        }

        // 返回token
        Ok(token)
    }
}

/// 实现qa微服务对应的接口
#[async_trait::async_trait]
impl QaService for QAServiceImpl {
    #[autometrics]
    async fn user_login(
        &self,
        request: Request<UserLoginRequest>,
    ) -> Result<Response<UserLoginReply>, Status> {
        let req = request.into_inner();
        println!("username:{}", req.username);
        let res = self.user_repo.fetch_one(&req.username).await;
        match res {
            Err(err) => {
                let err = err.downcast().expect("failed to convert into sqlx error");
                match err {
                    sqlx::Error::RowNotFound => {
                        // 用户不存在
                        info!("request user:{} not found", req.username);
                        Err(Status::new(
                            Code::Unknown,
                            format!("当前用户{}未注册", req.username),
                        ))
                    }
                    other => {
                        info!(
                            "login request user:{} server inner error:{}",
                            req.username, other
                        );
                        Err(Status::new(
                            Code::Internal,
                            format!("用户:{} 登录发生未知错误:{}", req.username, other),
                        ))
                    }
                }
            }
            Ok(user) => {
                let pwd = format!("{:x}", md5::compute(req.password.as_bytes()));
                if user.password != pwd {
                    return Err(Status::new(
                        Code::InvalidArgument,
                        format!("用户{}输入的密码错误", &req.username),
                    ));
                }

                // 登录成功，生成唯一标识token
                // 并将token作为cache key，value是UserSessionEntity
                let login_time = Local::now();
                let expired = login_time.timestamp() + 86400; // token过期时间
                let expire_time = Local.timestamp_opt(expired, 0).unwrap();
                let user_session = UserSessionEntity {
                    uid: user.id,
                    username: user.username,
                    openid: user.openid,
                    login_time: login_time.format("%Y-%m-%d %H:%M:%S").to_string(),
                    expire_time: expire_time.format("%Y-%m-%d %H:%M:%S").to_string(),
                };

                // 生成登录的唯一标识token
                let token = Uuid::new_v4().to_string().replace("-", "");
                let key = format!("user_login:{}", token);
                let _ = self.user_session_repo.set(&key, &user_session, 86400).await;
                let payload = format!("{}:{}", token, expired); // token:expired字符串加密

                // 返回payload加密字符串作为登录的唯一标识token
                let token = self
                    .aes_crypto
                    .encrypt(&payload)
                    .expect("failed to encrypt token");
                let reply = Response::new(UserLoginReply { token });
                Ok(reply)
            }
        }
    }

    // 退出登录操作，清除user session
    async fn user_logout(
        &self,
        request: Request<UserLogoutRequest>,
    ) -> Result<Response<UserLogoutReply>, Status> {
        let req = request.into_inner();
        println!("request encrypt token:{}", req.token);
        let login_res = self.check_token(&req.token);
        if let Err(err) = login_res {
            if err.to_string().contains("token has expired") {
                return Err(Status::new(
                    Code::Unauthenticated,
                    "token has expired".to_string(),
                ));
            }

            return Err(Status::new(
                Code::InvalidArgument,
                format!("parse token failed,error:{}", err),
            ));
        }

        let token = login_res.unwrap();
        let key = format!("user_login:{}", token);
        let res = self.user_session_repo.del(&key).await;
        if let Err(err) = res {
            return Err(Status::new(
                Code::Aborted,
                format!("user logout error:{}", err),
            ));
        }

        let reply = UserLogoutReply { state: 1 };
        Ok(Response::new(reply))
    }

    // 注册用户
    async fn user_register(
        &self,
        request: Request<UserRegisterRequest>,
    ) -> Result<Response<UserRegisterReply>, Status> {
        let req = request.into_inner();
        // 先判断用户是否存在
        let res = self.user_repo.check_user_exist(&req.username).await;
        match res {
            Ok(_) => {
                return Err(Status::new(
                    Code::AlreadyExists,
                    format!("当前用户{}已经存在", &req.username),
                ))
            }
            Err(err) => {
                // 将错误转换为原始类型
                let err = err.downcast().expect("failed to convert into sqlx error");
                match err {
                    sqlx::Error::RowNotFound => {
                        // 用户不存在就插入记录
                        let result = self.user_repo.add(&req.username, &req.password).await;
                        if let Err(err) = result {
                            return Err(Status::new(
                                Code::Unknown,
                                format!("当前用户{}注册失败，错误信息:{}", &req.username, err),
                            ));
                        }
                    }
                    other => {
                        info!(
                            "register request user:{} server inner error:{}",
                            req.username, other
                        );
                        return Err(Status::new(
                            Code::Internal,
                            format!("服务内部错误:{}", other),
                        ));
                    }
                }
            }
        }

        // 返回结果
        let reply = UserRegisterReply { state: 1 };
        Ok(Response::new(reply))
    }

    async fn verify_token(
        &self,
        request: Request<VerifyTokenRequest>,
    ) -> Result<Response<VerifyTokenReply>, Status> {
        let req = request.into_inner();
        println!("request_id:{} encrypt token:{}", req.request_id, req.token);
        let login_res = self.check_token(&req.token);
        if let Err(err) = login_res {
            if err.to_string().contains("token has expired") {
                return Err(Status::new(
                    Code::Unauthenticated,
                    "token has expired".to_string(),
                ));
            }

            return Err(Status::new(
                Code::InvalidArgument,
                format!("parse token failed,error:{}", err),
            ));
        }

        let token = login_res.unwrap();
        let key = format!("user_login:{}", token);
        let res = self.user_session_repo.get(&key).await;
        if res.is_err() {
            let err = res.err().unwrap();
            let err_msg = err.to_string();
            if err_msg.contains("session not found") || err_msg.contains("session is empty") {
                let reply = VerifyTokenReply {
                    state: 0,
                    reason: "login session not found".to_string(),
                    username: "".to_string(),
                };
                return Ok(Response::new(reply));
            }

            // 其他未知错误
            let reply = VerifyTokenReply {
                state: -1,
                reason: format!("unknown error:{}", err),
                username: "".to_string(),
            };
            return Ok(Response::new(reply));
        }

        let user = res.unwrap();
        info!("current token uid:{} username:{}", user.uid, user.username);

        // 验证成功
        let reply = VerifyTokenReply {
            state: 1,
            reason: "".to_string(),
            username: user.username,
        };

        Ok(Response::new(reply))
    }

    async fn add_question(
        &self,
        request: Request<AddQuestionRequest>,
    ) -> Result<Response<AddQuestionReply>, Status> {
        todo!()
    }

    async fn delete_question(
        &self,
        request: Request<DeleteQuestionRequest>,
    ) -> Result<Response<DeleteQuestionReply>, Status> {
        todo!()
    }

    async fn update_question(
        &self,
        request: Request<UpdateQuestionRequest>,
    ) -> Result<Response<UpdateQuestionReply>, Status> {
        todo!()
    }

    async fn question_detail(
        &self,
        request: Request<QuestionDetailRequest>,
    ) -> Result<Response<QuestionDetailReply>, Status> {
        todo!()
    }

    async fn latest_questions(
        &self,
        request: Request<LatestQuestionsRequest>,
    ) -> Result<Response<LatestQuestionsReply>, Status> {
        todo!()
    }

    async fn answer_list(
        &self,
        request: Request<AnswerListRequest>,
    ) -> Result<Response<AnswerListReply>, Status> {
        todo!()
    }

    async fn add_answer(
        &self,
        request: Request<AddAnswerRequest>,
    ) -> Result<Response<AddAnswerReply>, Status> {
        todo!()
    }

    async fn delete_answer(
        &self,
        request: Request<DeleteAnswerRequest>,
    ) -> Result<Response<DeleteAnswerReply>, Status> {
        todo!()
    }

    async fn update_answer(
        &self,
        request: Request<UpdateAnswerRequest>,
    ) -> Result<Response<UpdateAnswerReply>, Status> {
        todo!()
    }

    async fn answer_detail(
        &self,
        request: Request<AnswerDetailRequest>,
    ) -> Result<Response<AnswerDetailReply>, Status> {
        todo!()
    }

    async fn answer_agree(
        &self,
        request: Request<AnswerAgreeRequest>,
    ) -> Result<Response<AnswerAgreeReply>, Status> {
        todo!()
    }
}
