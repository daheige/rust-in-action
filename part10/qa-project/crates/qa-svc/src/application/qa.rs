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
use std::collections::HashMap;
use tonic::{Code, Request, Response, Status};
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
    // 验证token是否有效，返回用户唯一标识openid
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
        if arr.len() != 3 {
            return Err("token invalid".to_string());
        }

        let openid = arr[1].to_string();
        if openid.len() != 32 {
            return Err("token length invalid".to_string());
        }

        // 判断token是否过期
        let expired = arr[2].parse::<i64>();
        if let Err(err) = expired {
            return Err(format!("token expire_time parse error:{}", err));
        }
        let expire_time = expired.unwrap();
        let current_time = Local::now().timestamp();
        let elapsed = (current_time - expire_time).abs();
        if elapsed >= 86400 {
            return Err("token has expired".to_string());
        }

        // 返回openid
        Ok(openid)
    }
}

/// 实现qa微服务对应的接口
#[async_trait::async_trait]
impl QaService for QAServiceImpl {
    // 实现用户登录，如果用户登录成功，返回登录唯一标识token,否则返回对应的错误信息
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
                            format!("用户:{}登录发生未知错误:{}", req.username, other),
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

                // 判断是否已经登录过
                let key = format!("user_login:{}", user.openid);
                let res = self.user_session_repo.get(&key).await;
                if res.is_ok() {
                    return Err(Status::new(
                        Code::AlreadyExists,
                        format!("用户{}已经登录，请不要反复登录", &req.username),
                    ));
                }

                // 登录成功，生成唯一标识token
                // token=login_id:openid:expired字符串加密
                let login_id = Uuid::new_v4().to_string().replace("-", "");
                let login_time = Local::now();
                let expired = login_time.timestamp() + 86400; // token过期时间
                let payload = format!("{}:{}:{}", login_id, user.openid, expired);
                // 返回payload加密字符串作为登录的唯一标识token
                let token = self
                    .aes_crypto
                    .encrypt(&payload)
                    .expect("failed to encrypt token");

                // 设置登录session
                // 将openid作为cache key，value是UserSessionEntity
                let expire_time = Local.timestamp_opt(expired, 0).unwrap();
                let user_session = UserSessionEntity {
                    uid: user.id,
                    username: user.username,
                    openid: user.openid,
                    login_time: login_time.format("%Y-%m-%d %H:%M:%S").to_string(),
                    expire_time: expire_time.format("%Y-%m-%d %H:%M:%S").to_string(),
                };
                let _ = self.user_session_repo.set(&key, &user_session, 86400).await;
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

        let openid = login_res.unwrap();
        let key = format!("user_login:{}", openid);
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

    // 验证登录的token是否有效
    async fn verify_token(
        &self,
        request: Request<VerifyTokenRequest>,
    ) -> Result<Response<VerifyTokenReply>, Status> {
        let req = request.into_inner();
        println!("request_id:{} encrypt token:{}", req.request_id, req.token);
        let login_res = self.check_token(&req.token);
        if login_res.is_err() {
            let err = login_res.err().unwrap();
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

        let openid = login_res.unwrap();
        let key = format!("user_login:{}", openid);
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

    // 发表问题
    async fn add_question(
        &self,
        request: Request<AddQuestionRequest>,
    ) -> Result<Response<AddQuestionReply>, Status> {
        let req = request.into_inner();
        let question = QuestionsEntity {
            title: req.title,
            content: req.content,
            created_by: req.created_by,
            ..Default::default()
        };
        let res = self.question_repo.add(&question).await;
        if let Err(err) = res {
            return Err(Status::new(
                Code::Unknown,
                format!("failed to add question,error:{}", err),
            ));
        }

        let id = res.unwrap();
        let reply = AddQuestionReply { id };
        Ok(Response::new(reply))
    }

    // 删除问题
    async fn delete_question(
        &self,
        request: Request<DeleteQuestionRequest>,
    ) -> Result<Response<DeleteQuestionReply>, Status> {
        let req = request.into_inner();
        info!("request question id:{} username:{}", req.id, req.username);
        let res = self.question_repo.delete(req.id, &req.username).await;
        if let Err(err) = res {
            return Err(Status::new(
                Code::Unknown,
                format!("failed to delete question,error:{}", err),
            ));
        }

        let reply = DeleteQuestionReply { state: 1 };
        Ok(Response::new(reply))
    }

    // 更新问题
    async fn update_question(
        &self,
        request: Request<UpdateQuestionRequest>,
    ) -> Result<Response<UpdateQuestionReply>, Status> {
        let req = request.into_inner();
        info!(
            "request question id:{} updated_by:{}",
            req.id, req.updated_by
        );
        let question = QuestionsEntity {
            title: req.title,
            content: req.content,
            updated_by: req.updated_by,
            ..Default::default()
        };

        let res = self.question_repo.update(req.id, &question).await;
        if let Err(err) = res {
            return Err(Status::new(
                Code::Unknown,
                format!("failed to update question,error:{}", err),
            ));
        }

        let reply = UpdateQuestionReply { state: 1 };
        Ok(Response::new(reply))
    }

    // 查看问题详情
    async fn question_detail(
        &self,
        request: Request<QuestionDetailRequest>,
    ) -> Result<Response<QuestionDetailReply>, Status> {
        let req = request.into_inner();
        info!("request question id:{} username:{}", req.id, req.username);
        let res = self.question_repo.fetch_one(req.id).await;
        if let Err(err) = res {
            let err = err.downcast().unwrap();
            match err {
                sqlx::Error::RowNotFound => {
                    return Err(Status::new(
                        Code::NotFound,
                        "current question not found".to_string(),
                    ));
                }
                _ => {
                    info!("failed to query question id:{},error:{}", req.id, err);
                }
            }

            return Err(Status::new(
                Code::Unknown,
                "failed to query question".to_string(),
            ));
        }

        let entry = res.unwrap();
        // 增加问题浏览数
        let data = EntityReadCountData {
            target_id: entry.id,
            target_type: "question".to_string(),
            count: 1,
        };
        let mut read_count = entry.read_count as i64;
        let read_count_res = self.read_count_repo.incr(&data).await;
        if let Err(err) = read_count_res {
            info!(
                "failed to incr question id:{} read_count,error:{}",
                entry.id, err
            );
        } else {
            read_count += read_count_res.unwrap() as i64;
        }

        let read_count = read_count as u64;
        println!("question read_count:{}", read_count);

        let question = QuestionEntity {
            id: entry.id,
            title: entry.title,
            content: entry.content,
            created_by: entry.created_by,
            read_count,
            reply_count: entry.reply_count,
        };
        let reply = QuestionDetailReply {
            question: Some(question),
        };
        Ok(Response::new(reply))
    }

    // 最新问题列表
    async fn latest_questions(
        &self,
        request: Request<LatestQuestionsRequest>,
    ) -> Result<Response<LatestQuestionsReply>, Status> {
        let req = request.into_inner();
        info!(
            "request username:{} limit:{} last_id:{}",
            req.username, req.limit, req.last_id
        );

        let question_res = self
            .question_repo
            .latest_lists(req.last_id, req.limit)
            .await;
        if let Err(err) = question_res {
            return Err(Status::new(
                Code::Unknown,
                format!("failed to query questions,error:{}", err),
            ));
        }

        let res = question_res.unwrap();
        if res.questions.len() == 0 {
            let reply = LatestQuestionsReply {
                last_id: 0,
                is_end: true,
                list: vec![],
            };
            return Ok(Response::new(reply));
        }

        let mut questions = Vec::with_capacity(res.questions.len());
        for item in res.questions {
            let question = QuestionEntity {
                id: item.id,
                title: item.title,
                content: item.content,
                created_by: item.created_by,
                read_count: item.read_count,
                reply_count: item.reply_count,
            };
            questions.push(question);
        }

        let reply = LatestQuestionsReply {
            last_id: res.last_id.unwrap_or(0),
            is_end: res.is_end,
            list: questions,
        };
        Ok(Response::new(reply))
    }

    // 答案列表
    async fn answer_list(
        &self,
        request: Request<AnswerListRequest>,
    ) -> Result<Response<AnswerListReply>, Status> {
        let req = request.into_inner();
        info!(
            "request username:{} question_id:{} limit:{} page:{}",
            req.username, req.question_id, req.limit, req.page
        );
        let answer_res = self
            .answer_repo
            .lists(req.question_id, req.page, req.limit, "id desc")
            .await;
        if let Err(err) = answer_res {
            return Err(Status::new(
                Code::Unknown,
                format!("failed to query answers,error:{}", err),
            ));
        }

        let res = answer_res.unwrap();
        if res.total == 0 {
            let reply = AnswerListReply {
                list: vec![],
                total: 0,
                total_page: 0,
                page_size: req.limit,
                current_page: req.page,
                is_end: true,
            };
            return Ok(Response::new(reply));
        }

        // 批量获取当前请求用户对这一批回答是否点赞
        let ids: Vec<u64> = res.answers.iter().map(|item| item.id).collect();
        let vote_map = self
            .vote_repo
            .is_batch_voted(&ids, "answer", &req.username)
            .await
            .unwrap_or(HashMap::default());
        println!("vote_map:{:?}", vote_map);

        let mut answers = Vec::with_capacity(res.answers.len());
        for item in res.answers {
            let has_agreed = vote_map.contains_key(&item.id);
            let answer = AnswerEntity {
                id: item.id,
                question_id: item.question_id,
                content: item.content,
                created_by: item.created_by,
                agree_count: item.agree_count,
                has_agreed,
            };
            answers.push(answer);
        }

        let reply = AnswerListReply {
            list: answers,
            total: res.total,
            total_page: res.total_page,
            page_size: res.page_size,
            current_page: res.current_page,
            is_end: res.is_end,
        };

        Ok(Response::new(reply))
    }

    // 添加答案
    async fn add_answer(
        &self,
        request: Request<AddAnswerRequest>,
    ) -> Result<Response<AddAnswerReply>, Status> {
        let req = request.into_inner();
        if req.answer.is_none() {
            return Err(Status::new(
                Code::InvalidArgument,
                "answer field is empty".to_string(),
            ));
        }

        let answer = req.answer.unwrap();
        info!(
            "request username:{} question_id:{}",
            answer.created_by, answer.question_id,
        );
        let res = self
            .answer_repo
            .add(&AnswersEntity {
                question_id: answer.question_id,
                content: answer.content,
                created_by: answer.created_by,
                ..Default::default()
            })
            .await;
        if let Err(err) = res {
            return Err(Status::new(
                Code::Unknown,
                format!("failed to add answer,error:{}", err),
            ));
        }

        let id = res.unwrap_or(0);
        let reply = AddAnswerReply { id };
        Ok(Response::new(reply))
    }

    // 删除答案
    async fn delete_answer(
        &self,
        request: Request<DeleteAnswerRequest>,
    ) -> Result<Response<DeleteAnswerReply>, Status> {
        let req = request.into_inner();
        info!("request username:{} answer_id:{}", req.username, req.id);
        let res = self.answer_repo.delete(req.id, &req.username).await;
        if let Err(err) = res {
            return Err(Status::new(
                Code::Unknown,
                format!("failed to delete answer,error:{}", err),
            ));
        }

        let reply = DeleteAnswerReply { state: 1 };
        Ok(Response::new(reply))
    }

    // 更新答案
    async fn update_answer(
        &self,
        request: Request<UpdateAnswerRequest>,
    ) -> Result<Response<UpdateAnswerReply>, Status> {
        let req = request.into_inner();
        info!("request username:{} answer_id:{}", req.username, req.id);
        let res = self
            .answer_repo
            .update(req.id, &req.content, &req.username)
            .await;
        if let Err(err) = res {
            return Err(Status::new(
                Code::Unknown,
                format!("failed to update answer,error:{}", err),
            ));
        }

        let reply = UpdateAnswerReply { state: 1 };
        Ok(Response::new(reply))
    }

    // 回答详情
    async fn answer_detail(
        &self,
        request: Request<AnswerDetailRequest>,
    ) -> Result<Response<AnswerDetailReply>, Status> {
        let req = request.into_inner();
        info!("request username:{} answer_id:{}", req.username, req.id);
        let res = self.answer_repo.fetch_one(req.id).await;
        if let Err(err) = res {
            let err = err.downcast().unwrap();
            match err {
                sqlx::Error::RowNotFound => {
                    return Err(Status::new(
                        Code::NotFound,
                        "current answer not found".to_string(),
                    ));
                }
                _ => {
                    info!("failed to query answer id:{},error:{}", req.id, err);
                }
            }

            return Err(Status::new(
                Code::Unknown,
                "failed to query answer".to_string(),
            ));
        }

        let answer = res.unwrap();
        let has_agreed = self
            .vote_repo
            .is_voted(req.id, "answer", &req.username)
            .await
            .unwrap_or(false);
        let answer_entry = AnswerEntity {
            id: answer.id,
            question_id: answer.question_id,
            content: answer.content,
            created_by: answer.created_by,
            agree_count: answer.agree_count,
            has_agreed,
        };
        let reply = AnswerDetailReply {
            answer: Some(answer_entry),
        };
        Ok(Response::new(reply))
    }

    // 回答点赞
    async fn answer_agree(
        &self,
        request: Request<AnswerAgreeRequest>,
    ) -> Result<Response<AnswerAgreeReply>, Status> {
        let req = request.into_inner();
        info!("request username:{} answer_id:{}", req.created_by, req.id);
        let answer_res = self.answer_repo.fetch_one(req.id).await;
        if let Err(err) = answer_res {
            return Err(Status::new(
                Code::Unknown,
                format!("failed to query answer,error:{}", err),
            ));
        }

        // 判断是否点赞过
        let is_voted = self
            .vote_repo
            .is_voted(req.id, "answer", &req.created_by)
            .await
            .unwrap_or(false);
        if req.action == "up" {
            if is_voted {
                let reply = AnswerAgreeReply {
                    state: 0,
                    reason: "you already voted it".to_string(),
                    agree_count: 0,
                };
                return Ok(Response::new(reply));
            }
        } else {
            if !is_voted {
                let reply = AnswerAgreeReply {
                    state: 0,
                    reason: "You didn't vote it,bad request".to_string(),
                    agree_count: 0,
                };
                return Ok(Response::new(reply));
            }
        }

        // 是否要返回点赞数，可根据实际业务场景决定
        let mut agree_count = answer_res.unwrap().agree_count as i64;
        if req.action == "up" {
            agree_count += 1;
        } else {
            if agree_count >= 1 {
                agree_count -= 1;
            }
        }
        let msg = VoteMessage {
            target_id: req.id,
            target_type: "answer".to_string(),
            created_by: req.created_by,
            action: req.action,
        };
        let res = self.vote_repo.publish(msg).await;
        if let Err(err) = res {
            return Err(Status::new(
                Code::Unknown,
                format!("failed to vote answer,error:{}", err),
            ));
        }

        let reply = AnswerAgreeReply {
            state: 1,
            reason: "success".to_string(),
            agree_count: agree_count as u64,
        };

        Ok(Response::new(reply))
    }
}
