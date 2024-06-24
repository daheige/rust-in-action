use chrono::Local;
use sqlx::{MySql, Pool};
use crate::domain::entity::{QuestionsEntity, UsersEntity};
use crate::domain::repository::QuestionRepo;

// QuestionRepo 具体实现
struct QuestionRepoImpl {
    mysql_pool: sqlx::MySqlPool,
}

pub fn new_question_repo(mysql_pool: Pool<MySql>) -> impl QuestionRepo {
    QuestionRepoImpl { mysql_pool }
}

#[async_trait::async_trait]
impl QuestionRepo for QuestionRepoImpl {
    async fn add(&self, question: &QuestionsEntity) -> anyhow::Result<u64> {
        let sql = format!(
            "insert into {} (title,content,created_by,created_at) value(?,?,?,?)",
            QuestionsEntity::table_name()
        );

        let created_at = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let affect_rows = sqlx::query(&sql)
            .bind(&question.title)
            .bind(&question.content)
            .bind(&question.created_by)
            .bind(created_at)
            .execute(&self.mysql_pool)
            .await?;

        let id = affect_rows.last_insert_id();
        println!("current insert question id = {}", id);

        Ok(id)
    }

    async fn delete(&self, id: u64, username: &str) -> anyhow::Result<()> {
        let sql = format!(
            "delete from {} where id = ? and created_by = ?",
            QuestionsEntity::table_name()
        );
        let affect_rows = sqlx::query(&sql)
            .bind(id)
            .bind(username)
            .execute(&self.mysql_pool)
            .await?;
        println!("affected rows: {}", affect_rows.rows_affected());

        Ok(())
    }

    async fn update(&self, id: u64, question: &mut QuestionsEntity) -> anyhow::Result<()> {
        todo!()
    }

    async fn fetch_one(&self, id: u64) -> anyhow::Result<QuestionsEntity> {
        let sql = format!(
            "select * from {} where id = ?",
            QuestionsEntity::table_name(),
        );

        // query_as将其映射到结构体中
        let question: QuestionsEntity = sqlx::query_as(&sql)
            .bind(id)
            .fetch_one(&self.mysql_pool)
            .await?;

        Ok(question)
    }

    async fn lists(
        &self,
        last_id: i64,
        limit: i64,
        order_by: String,
    ) -> anyhow::Result<Vec<QuestionsEntity>> {
        todo!()
    }
}
