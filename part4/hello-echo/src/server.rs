use tide::{Body, Request, Response, StatusCode};
use tide::log;
use serde::{Deserialize, Serialize};

#[async_std::main]
async fn main() -> tide::Result<()>{
    log::start();

    let address = "127.0.0.1:8080";
    println!("server run on:{}",address);

    // 创建tide app实例
    let mut app = tide::new();

    // 设置请求的日志中间件
    app.with(log::LogMiddleware::new());

    // 绑定路由
    app.at("/hello").get(hello);
    app.at("/cat").post(cat);

    // 启动web服务
    app.listen(address).await?;
    Ok(())
}

// 请求参数Request，并返回tide::Result
// 由于不需要接收请求参数，所以这个Request提取数据为()空元组
async fn hello(mut _req: Request<()>)->tide::Result{
    Ok("hello,world".into())
}

#[derive(Deserialize, Serialize)]
struct Cat {
    name: String,
    id:i64,
}

// 接收body json格式，并返回json格式
async fn cat(mut req: Request<()>)->tide::Result{
    let cat : Cat = req.body_json().await?;
    println!("cat.name:{}",cat.name);
    let mut res = Response::new(StatusCode::Ok); // 创建一个response
    if cat.id <= 0{
        res.set_body("bad request");
    }else{
        res.set_body(Body::from_json(&cat)?);
    }

    Ok(res)
}
