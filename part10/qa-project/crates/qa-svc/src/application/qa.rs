use crate::config::AppState;
use crate::domain::entity::{AnswersEntity, EntityReadCountData, QuestionsEntity, VoteMessage};
use crate::domain::repository::{AnswerRepo, QuestionRepo, ReadCountRepo, UserRepo, UserVoteRepo};
use crate::infrastructure::persistence::{new_answer_repo, new_question_repo, new_user_repo};
use crate::infrastructure::read_count::new_read_count_repo;
use crate::infrastructure::vote::new_vote_repo;
use autometrics::autometrics;
use pb::qa::qa_service_server::QaService;
use pb::qa::*;
use sqlx::types::chrono::NaiveDateTime;
use sqlx::FromRow;
use tonic::{Code, Request, Response, Status};
use uuid::Uuid;

/// 实现qa.proto 接口服务
struct QAServiceImpl {
    user_repo: Box<dyn UserRepo>,
    question_repo: Box<dyn QuestionRepo>,
    answer_repo: Box<dyn AnswerRepo>,
    read_count_repo: Box<dyn ReadCountRepo>,
    vote_repo: Box<dyn UserVoteRepo>,
}

// 创建QaService实例
pub fn new_qa_service(app_state: AppState) -> impl QaService {
    let user_repo = Box::new(new_user_repo(app_state.mysql_pool.clone()));
    let question_repo = Box::new(new_question_repo(app_state.mysql_pool.clone()));
    let answer_repo = Box::new(new_answer_repo(app_state.mysql_pool.clone()));
    let read_count_repo = Box::new(new_read_count_repo(
        app_state.redis_pool,
        app_state.mysql_pool.clone(),
    ));
    let vote_repo = Box::new(new_vote_repo(app_state.mysql_pool, app_state.pulsar_client));
    QAServiceImpl {
        user_repo,
        question_repo,
        answer_repo,
        read_count_repo,
        vote_repo,
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

        // let usernames = vec!["daheige"];
        // let users = self.user_repo.batch_users(usernames).await;
        // println!("users:{:?}",users);

        // let question = self.question_repo.fetch_one(1).await;
        // println!("question:{:?}",question);

        // let mut question = QuestionsEntity::default();
        // question.title = "abc".to_string();
        // question.content = "abc".to_string();
        // question.updated_by = "daheige".to_string();
        // let res = self.question_repo.update(1,&question).await;

        // let questions = self.question_repo.latest_lists(1, 1).await;
        // println!("latest questions:{:?}", questions);

        // let res = self.user_repo.check_user_exist("xiaoming").await;
        // if let Ok(exist) = res {
        //     println!("exist:{}", exist);
        // } else {
        //     println!("user not found");
        // }

        // let mut answer = AnswersEntity::default();
        // answer.question_id = 1;
        // answer.content = "abc".to_string();
        // answer.created_by = "daheige".to_string();
        // let res = self.answer_repo.add(&answer).await;

        // let res = self.answer_repo.delete(1,"daheige").await;
        // let res = self.answer_repo.fetch_one(20).await;
        // if res.is_ok(){
        //     println!("res:{:?}",res.unwrap());
        // }else{
        //     let err : sqlx::Error = res.err().unwrap().downcast().unwrap();
        //     println!("err: {}",err);
        // }

        // let res = self.answer_repo.update(2,"hello","daheige").await;
        // let res = self.answer_repo.lists(2,1,2,"id desc").await;

        // let res = self.user_repo.check_user_exist("lisi").await;
        // println!("res:{:?}",res.is_ok());
        // let data = EntityReadCountData{
        //     target_id:1,
        //     target_type:"question".to_string(),
        //     count:1,
        // };
        // let res = self.read_count_repo.incr(&data).await;
        // let res = self.read_count_repo.handler("question").await;
        // println!("res:{:?}",res);

        // answer vote
        // let res = self.vote_repo.publish(VoteMessage {
        //     target_type: "answer".to_string(),
        //     target_id: 3,
        //     created_by: "daheige".to_string(),
        //     action: "cancel".to_string(),
        // }).await;
        // println!("res:{:?}", res);

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
