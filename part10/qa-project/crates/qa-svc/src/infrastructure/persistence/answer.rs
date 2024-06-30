use crate::domain::entity::{AnswerListReply, AnswersEntity, CountInfo};
use crate::domain::repository::AnswerRepo;
use chrono::Local;

struct AnswerRepoImpl {
    mysql_pool: sqlx::MySqlPool,
}

pub fn new_answer_repo(mysql_pool: sqlx::MySqlPool) -> impl AnswerRepo {
    AnswerRepoImpl { mysql_pool }
}

#[async_trait::async_trait]
impl AnswerRepo for AnswerRepoImpl {
    async fn add(&self, answer: &AnswersEntity) -> anyhow::Result<u64> {
        let sql = format!(
            "insert into {} (question_id,content,created_by,created_at) value(?,?,?,?)",
            AnswersEntity::table_name()
        );

        let created_at = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let affect_rows = sqlx::query(&sql)
            .bind(&answer.question_id)
            .bind(&answer.content)
            .bind(&answer.created_by)
            .bind(created_at)
            .execute(&self.mysql_pool)
            .await?;

        let id = affect_rows.last_insert_id();
        println!("current insert answer id = {}", id);

        Ok(id)
    }

    async fn delete(&self, id: u64, username: &str) -> anyhow::Result<()> {
        let sql = format!(
            "delete from {} where id = ? and created_by = ?",
            AnswersEntity::table_name()
        );
        let affect_rows = sqlx::query(&sql)
            .bind(id)
            .bind(username)
            .execute(&self.mysql_pool)
            .await?;
        println!("affected rows: {}", affect_rows.rows_affected());

        Ok(())
    }

    async fn update(&self, id: u64, content: &str, updated_by: &str) -> anyhow::Result<()> {
        let sql = format!(
            r#"update {} set content = ?,updated_by = ?,updated_at = ? where id = ?"#,
            AnswersEntity::table_name(),
        );

        println!("update sql:{}", sql);
        let updated_at = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let affect_res = sqlx::query(&sql)
            .bind(content)
            .bind(updated_by)
            .bind(updated_at)
            .bind(id)
            .execute(&self.mysql_pool)
            .await?;
        println!(
            "current answer affected_rows = {}",
            affect_res.rows_affected()
        );
        Ok(())
    }

    async fn fetch_one(&self, id: u64) -> anyhow::Result<AnswersEntity> {
        let sql = format!(
            "select * from {} where id = ? limit 1",
            AnswersEntity::table_name()
        );

        // query_as将其映射到结构体中
        let answer: AnswersEntity = sqlx::query_as(&sql)
            .bind(id)
            .fetch_one(&self.mysql_pool)
            .await?;
        Ok(answer)
    }

    async fn lists(
        &self,
        question_id: u64,
        page: u64,
        limit: u64,
        order_by: &str,
    ) -> anyhow::Result<AnswerListReply> {
        let sql = format!(
            "select count(*) as cnt from {} where question_id = ?",
            AnswersEntity::table_name(),
        );
        let res: CountInfo = sqlx::query_as(&sql)
            .bind(question_id)
            .fetch_one(&self.mysql_pool)
            .await?;
        // mysql对应的bigint类型，对应rust的i64，这里使用as将其转换为u64类型
        let total = res.cnt as u64;
        if total == 0 {
            let reply = AnswerListReply::new(vec![], total, page, limit);
            return Ok(reply);
        }

        let sql = format!(
            "select * from {} where question_id = ? order by {} limit {} offset {}",
            AnswersEntity::table_name(),
            order_by,
            limit,
            (page - 1) * limit,
        );
        // query_as将其映射到vec中
        let answers: Vec<AnswersEntity> = sqlx::query_as(&sql)
            .bind(question_id)
            .fetch_all(&self.mysql_pool)
            .await?;
        let reply = AnswerListReply::new(answers, total, page, limit);
        Ok(reply)
    }
}
