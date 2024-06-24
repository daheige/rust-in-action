use crate::config::AppState;
use crate::domain::repository::{QuestionRepo, UserRepo};
use crate::infrastructure::persistence::{new_user_repo,new_question_repo};
use autometrics::autometrics;
use chrono::{DateTime, Local};
use pb::qa::qa_service_server::QaService;
use pb::qa::*;
use pulsar::proto::schema::Type::LocalDateTime;
use sqlx::types::chrono::NaiveDateTime;
use sqlx::FromRow;
use tonic::{Code, Request, Response, Status};
use uuid::Uuid;
use crate::domain::entity::QuestionsEntity;

/// 实现qa.proto 接口服务
struct QAServiceImpl {
    user_repo: Box<dyn UserRepo>,
    question_repo: Box<dyn QuestionRepo>,
}

// 创建QaService实例
pub fn new_qa_service(app_state: AppState) -> impl QaService {
    let user_repo = Box::new(new_user_repo(app_state.mysql_pool.clone()));
    let question_repo = Box::new(new_question_repo(app_state.mysql_pool.clone()));
    QAServiceImpl { user_repo,question_repo }
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
        let reply = UserLoginReply {
            token: "abc".to_string(),
        };


        // let mut question = QuestionsEntity::default();
        // question.title = "abc".to_string();
        // question.content = "hello".to_string();
        // question.created_by = "daheige".to_string();
        //
        // let res = self.question_repo.add(&question).await;
        //
        // println!("res:{:?}",res);

        // let _ = self.question_repo.delete(2,"daheige").await;


        Ok(Response::new(reply))
    }

    async fn user_logout(
        &self,
        request: Request<UserLogoutRequest>,
    ) -> Result<Response<UserLogoutReply>, Status> {
        todo!()
    }

    // 注册用户
    async fn user_register(
        &self,
        request: Request<UserRegisterRequest>,
    ) -> Result<Response<UserRegisterReply>, Status> {
        let req = request.into_inner();
        // 先判断用户是否存在
        let res = self.user_repo.check_user(&req.username).await;
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
                        let result = self
                            .user_repo
                            .add(&req.username, &req.password)
                            .await;
                        if let Err(err) = result {
                            return Err(Status::new(
                                Code::Unknown,
                                format!("当前用户{}注册失败:{}", &req.username, err),
                            ));
                        }
                    }
                    other => {
                        println!("err:{}", other);
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

    async fn answer_agree(
        &self,
        request: Request<AnswerAgreeRequest>,
    ) -> Result<Response<AnswerAgreeReply>, Status> {
        todo!()
    }

    async fn answer_detail(
        &self,
        request: Request<AnswerDetailRequest>,
    ) -> Result<Response<AnswerDetailReply>, Status> {
        todo!()
    }
}
