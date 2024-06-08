use pb::qa::qa_service_server::QaService;
use pb::qa::*;
use tonic::{transport::Server, Request, Response, Status};

/// 实现hello.proto 接口服务
#[derive(Debug, Default)]
pub struct QAServiceImpl {}

impl QAServiceImpl {
    pub fn new() -> Self {
        Self {}
    }
}

/// 实现qa微服务对应的接口
#[tonic::async_trait]
impl QaService for QAServiceImpl {
    async fn user_login(
        &self,
        request: Request<UserLoginRequest>,
    ) -> Result<Response<UserLoginReply>, Status> {
        let req = &request.into_inner();
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

    async fn user_register(
        &self,
        request: Request<UserRegisterRequest>,
    ) -> Result<Response<UserRegisterReply>, Status> {
        todo!()
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
