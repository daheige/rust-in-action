use chrono::prelude::*;
use futures::TryStreamExt;
// 引入sqlx包
use sqlx::mysql::{MySqlPoolOptions, MySqlRow};
use sqlx::Row;

use std::env;
use std::ops::DerefMut;
use std::time::Duration;
use tokio;

// 定义users表字段的数据结构信息
// sqlx::FromRow 标记特征用于行记录读取
#[derive(Debug, sqlx::FromRow)]
struct UserEntity {
    id: i64,
    name: String,
    age: i32,
    id_card: String,
    last_update: NaiveDate, // 时间格式
}

// main入口函数，通过tokio运行时来执行sqlx异步的相关操作
#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // 数据库连接dsn句柄信息
    let default_dsn = "mysql://root:root123456@localhost/memberinfo";
    let dsn = env::var("DATABASE_URL").unwrap_or(default_dsn.to_string());

    // 创建数据库连接池
    let pool = MySqlPoolOptions::new()
        .max_connections(200) // 最大连接数
        .min_connections(5) // 最小连接数
        .max_lifetime(Duration::from_secs(1800)) // 最大生命周期
        .idle_timeout(Duration::from_secs(600)) // 空闲连接的生命周期
        .acquire_timeout(Duration::from_secs(10)) // 连接超时
        .connect(&dsn)
        .await?;

    // ====数据插入操作====
    // 1、使用execute插入数据
    // 通过r#前缀标记这是一个sql原始字符串语句，不需要转义
    // 默认情况下，SQLx提倡使用绑定参数，这对于防止SQL注入非常重要
    let sql = r#"insert into users (name,age,id_card,last_update) value(?,?,?,?)"#;
    let affect_rows = sqlx::query(sql)
        .bind("zhangsan") // 通过bind方法实现参数绑定
        .bind(33)
        .bind("abc")
        .bind(chrono::NaiveDate::from_ymd_opt(2022, 04, 13))
        .execute(&pool) // 异步执行sql
        .await?;
    // 获取插入的自增id
    let id = affect_rows.last_insert_id();
    println!("insert user id = {}", id);

    let sql = r#"insert into users (name,age,id_card,last_update) value(?,?,?,?)"#;
    let affect_rows = sqlx::query(sql)
        .bind("xiaoming")
        .bind(23)
        .bind("efg")
        .bind(chrono::NaiveDate::from_ymd_opt(2024, 02, 13))
        .execute(&pool)
        .await?;
    let id = affect_rows.last_insert_id();
    println!("current insert user id = {}", id);

    // ====数据查询操作====
    // 2、使用fetch获取多行数据并通过try_next方法遍历数据
    let sql = "select * from users where id >= ?";
    let mut rows = sqlx::query(sql).bind(1).fetch(&pool);
    while let Some(row) = rows.try_next().await? {
        let user = UserEntity {
            id: row.get("id"), // 在row上面通过get方法获取字段对应的值
            name: row.get("name"),
            age: row.get("age"),
            id_card: row.get("id_card"),
            last_update: row.get("last_update"),
        };

        println!("user = {:?}", user);
        println!(
            "user id = {} name = {} age = {} id_card = {} last_update = {}",
            user.id, user.name, user.age, user.id_card, user.last_update
        );
    }

    // ====数据查询操作====
    // 3、使用fetch加map方法传入闭包的形式读取数据
    // The fetch query finalizer returns a stream-like type
    // that iterates through the rows in the result sets.
    let sql = "select * from users where id >= ?";
    let records = sqlx::query(sql)
        .bind(1)
        .map(|row: MySqlRow| UserEntity {
            // 这个闭包的参数需要指定row数据为MySqlRow
            id: row.get("id"),
            name: row.get("name"),
            age: row.get("age"),
            id_card: row.get("id_card"),
            last_update: row.get("last_update"),
        })
        .fetch(&pool);
    // 将异步执行的结果通过pin的方式放入records集合中
    tokio::pin!(records);
    // 遍历数据
    while let Some(s) = records.try_next().await? {
        println!("s = {:?}", s);
    }

    // ====数据更新操作====
    // 4、使用execute，执行更新操作，返回 affect_rows 影响的行数
    // the executor query finalizer returns the number of affected rows,
    // if any, and drops all received results. In addition, there are fetch,
    // fetch_one, fetch_optional, and fetch_all to receive results.
    let sql = r#"update users set name = ? where id = ?"#;
    let affect_rows = sqlx::query(sql)
        .bind("zhangsan2")
        .bind(1)
        .execute(&pool)
        .await?;
    println!("{:?}", affect_rows);

    // =====query_as方法使用，将查询结果集转化为struct====
    // 5、使用fetch获取结果集Vec数据
    // To assist with mapping the row into a domain type,
    let sql = "select * from users where id >= ?";
    let mut stream = sqlx::query_as::<_, UserEntity>(sql).bind(1).fetch(&pool);
    while let Some(user) = stream.try_next().await? {
        println!("{:?}", user);
    }

    // 6、使用fetch_one获取一条结果集，并将其映射到结构体UserEntity中
    let sql = "select * from users where id = ?";
    let user: UserEntity = sqlx::query_as(sql).bind(1).fetch_one(&pool).await?;
    println!("user: {:?}", user);
    println!("id = {} name = {}", user.id, user.name);

    // 7、使用fetch_all获取多条记录，将所有的结果集放到Vec
    // 在使用fetch_all时，你必须从一个迭代器中获取每一行记录
    // SQLx为我们提供了一个宏Sqlx::FromRow，以便我们能够从SQL行向量中提取数据到结构体中
    // 您可以使用 query_as 将返回结果绑定到Vec中
    // 前提是你已经在UserEntity上面使用了标记属性：#[derive(Sqlx::FromRow)]
    let sql = "select * from users where id >= ?";
    let records: Vec<UserEntity> = sqlx::query_as(sql).bind(1).fetch_all(&pool).await?;
    for row in records {
        println!("current row = {:?}", row);
        println!("id = {} name = {}", row.id, row.name);
    }

    // 8、事务操作 begin/commit
    let sql = r#"insert into users (name,age,id_card,last_update) value(?,?,?,?)"#;
    let mut tx = pool.begin().await?; // 创建一个事务transaction
    let affect_rows = sqlx::query(sql)
        .bind("lisi")
        .bind(32)
        .bind("abc")
        .bind(chrono::NaiveDate::from_ymd_opt(2022, 04, 13))
        // In 0.7, `Transaction` can no longer implement `Executor` directly,
        // so it must be dereferenced to the internal connection type.
        // 这里需要对tx进行解引用并获取内部DB的可变引用connection
        .execute(tx.deref_mut())
        .await?;
    if affect_rows.rows_affected() == 0 {
        tx.rollback().await?;
    } else {
        tx.commit().await?; // 提交事务
        let id = affect_rows.last_insert_id(); // 获取插入的id
        println!("id = {}", id);
    }

    Ok(())
}
