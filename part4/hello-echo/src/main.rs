use tide::log;
use tide::Request;

// 指定async_std异步运行时
#[async_std::main]
async fn main() -> tide::Result<()> {
    // 启动日志组件
    log::start();

    let address = "127.0.0.1:8080";
    println!("server run on:{}", address);

    // 创建tide app实例
    let mut app = tide::new();

    // 设置请求的日志中间件
    app.with(log::LogMiddleware::new());

    // 绑定路由
    app.at("/").get(home);
    app.at("/hello/:name").get(hello);

    // 启动web服务
    app.listen(address).await?;
    Ok(())
}

// 该路由处理器函数的请求参数是tide::Request类型，返回结果是tide::Result类型。
async fn hello(req: Request<()>) -> tide::Result {
    println!("request method:{:?}", req.header("user-agent"));
    // 接收参数name
    let name = req.param("name").unwrap_or("world");
    Ok(format!("hello,{}", name).into())
}

// 由于不需要接收请求参数，所以这个Request提取数据为()空元组。
async fn home(_req: Request<()>) -> tide::Result {
    Ok("hello,tide".into())
}
