use chrono::Local;
use sqlx::{MySql, Pool};

use crate::domain::entity::{LatestQuestions, QuestionsEntity};
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
    // 发表问题
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

    // 修改问题
    async fn update(&self, id: u64, question: &QuestionsEntity) -> anyhow::Result<()> {
        let sql = format!(
            r#"update {} set title = ?,content = ?,updated_by = ?,updated_at = ? where id = ?"#,
            QuestionsEntity::table_name(),
        );

        println!("update sql:{}", sql);
        let updated_at = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let affect_res = sqlx::query(&sql)
            .bind(&question.title)
            .bind(&question.content)
            .bind(&question.updated_by)
            .bind(updated_at)
            .bind(id)
            .execute(&self.mysql_pool)
            .await?;

        println!(
            "current question affected_rows = {}",
            affect_res.rows_affected()
        );
        Ok(())
    }

    // 删除问题
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

    // 根据id查询问题实体
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

    // 最新问题列表
    async fn latest_lists(&self, last_id: u64, limit: u64) -> anyhow::Result<LatestQuestions> {
        let mut questions: Vec<QuestionsEntity> = vec![];
        if last_id > 0 {
            let sql = format!(
                "select * from {} where id < ? order by id desc limit ?",
                QuestionsEntity::table_name(),
            );
            // query_as将其映射到vec中
            questions = sqlx::query_as(&sql)
                .bind(last_id)
                .bind(limit)
                .fetch_all(&self.mysql_pool)
                .await?;
        } else {
            let sql = format!(
                "select * from {} order by id desc limit ?",
                QuestionsEntity::table_name(),
            );
            // query_as将其映射到vec中
            questions = sqlx::query_as(&sql)
                .bind(limit)
                .fetch_all(&self.mysql_pool)
                .await?;
        }

        let num = questions.len() as u64;
        let is_end = num < limit;
        let mut last_id: Option<u64> = None;
        if !is_end {
            // 数据没有到底
            last_id = Some(questions.last().unwrap().id);
        }

        let reply = LatestQuestions {
            questions,
            is_end,
            last_id,
        };

        Ok(reply)
    }
}
