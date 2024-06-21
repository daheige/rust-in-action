use crate::config::AppState;
use crate::entity::users::UsersEntity;
use autometrics::autometrics;
use chrono::{DateTime, Local};
use pb::qa::qa_service_server::QaService;
use pb::qa::*;
use pulsar::proto::schema::Type::LocalDateTime;
use sqlx::types::chrono::NaiveDateTime;
use sqlx::FromRow;
use tonic::{Code, Request, Response, Status};
use uuid::Uuid;

/// 实现qa.proto 接口服务
pub struct QAServiceImpl {
    app_state: AppState,
}

impl QAServiceImpl {
    pub fn new(app_state: AppState) -> Self {
        Self { app_state }
    }

    // 检查用户是否存在
    async fn check_user(&self, username: &str) -> Result<bool, sqlx::Error> {
        let sql = format!(
            "select * from {} where username = ?",
            UsersEntity::table_name(),
        );

        // query_as将其映射到结构体UserEntity中
        let user: UsersEntity = sqlx::query_as(&sql)
            .bind(username)
            .fetch_one(&self.app_state.mysql_pool)
            .await?;

        Ok(user.id > 0)
    }

    // 插入用户
    async fn insert_user(&self, username: &str, password: &str) -> Result<(), sqlx::Error> {
        let sql = r#"insert into users (username,password,openid,created_at) value(?,?,?,?)"#;
        let created_at = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let pwd = format!("{:x}", md5::compute(password.as_bytes()));
        let openid = Uuid::new_v4().to_string().replace("-", "");
        let affect_rows = sqlx::query(sql)
            .bind(username)
            .bind(pwd)
            .bind(openid)
            .bind(created_at)
            .execute(&self.app_state.mysql_pool)
            .await?;

        let id = affect_rows.last_insert_id();
        println!("current insert user id = {}", id);

        Ok(())
    }
}

/// 实现qa微服务对应的接口
#[tonic::async_trait]
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
        let res = self.check_user(&req.username).await;
        match res {
            Ok(_) => {
                return Err(Status::new(
                    Code::AlreadyExists,
                    format!("当前用户{}已经存在", &req.username),
                ))
            }
            Err(err) => {
                match err {
                    sqlx::Error::RowNotFound => {
                        // 用户不存在就插入记录
                        let result = self.insert_user(&req.username, &req.password).await;
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

    async fn question_read_count(
        &self,
        request: Request<QuestionReadCountRequest>,
    ) -> Result<Response<QuestionReadCountReply>, Status> {
        todo!()
    }
}
