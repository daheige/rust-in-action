use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: u64,
    username: String,
    nick: String,
}

fn main() {
    let user = User {
        id: 1,
        username: "daheige".to_string(),
        nick: "alex".to_string(),
    };

    // 使用json宏序列化操作，返回值是serde_json::value对象
    let res = json!(user);
    println!(
        "id:{} nick:{} name:{}",
        res["id"], res["nick"], res["username"],
    );
    // 在res上面调用to_string方法，将其转换为JSON字符串格式
    println!("res json encode:{}\n", res.to_string());

    println!("serialize using JSON literals");
    // 下面的代码没有提前定义数据类型，而是通过JSON字面量方式序列化，
    // 返回值同样是serde_json::value对象
    let id = 1;
    let res = json!({
        "id":id,
        "username":"daheige",
        "nick":"alex",
    });
    // 通过方括号索引方式来访问字段
    println!(
        "id:{} nick:{} username:{}",
        res["id"], res["nick"], res["username"]
    );
    let id = res["id"].as_u64().unwrap(); // 类型转换
    println!("id:{}", id);

    // 在res上面调用to_string方法，将其转换为JSON字符串格式
    println!("res json encode:{}", res.to_string());
}
