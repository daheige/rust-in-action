# Interceptor拦截器使用

```rust
   // create grpc service
let qa_service = application::new_qa_service(app_state);
let svc = QaServiceServer::new(qa_service);
// 通过layer方法设置自定义中间件 MyMiddlewareLayer，这种方式可以通过链式调用绑定多个中间件
let grpc_server = Server::builder()
.layer(LoggerMiddlewareLayer)
.add_service(reflection_service)
.add_service(svc)
.serve_with_shutdown(
address,
graceful_shutdown(Duration::from_secs(APP_CONFIG.graceful_wait_time)),
);

// let svc = QaServiceServer::new(qa_service);
// // 通过layer方法和InterceptorLayer::new设置自定义中间件 LoggingInterceptor，
// // 这种方式可以通过链式调用绑定多个中间件
// let grpc_server = Server::builder()
//     .layer(InterceptorLayer::new(LoggingInterceptor))
//     .add_service(reflection_service)
//     .add_service(svc)
//     .serve_with_shutdown(
//         address,
//         graceful_shutdown(Duration::from_secs(APP_CONFIG.graceful_wait_time)),
//     );

// 也可以直接通过 with_interceptor 方法设置中间件
// let svc = QaServiceServer::with_interceptor(qa_service, LoggingInterceptor);
// let grpc_server = Server::builder()
//     .add_service(reflection_service)
//     .add_service(svc)
//     .serve_with_shutdown(
//         address,
//         graceful_shutdown(Duration::from_secs(APP_CONFIG.graceful_wait_time)),
//     );

// 如果不需拦截器，使用下面方式定义
// let svc = QaServiceServer::new(qa_service);
// let grpc_server = Server::builder()
//     .add_service(reflection_service)
//     .add_service(svc)
//     .serve_with_shutdown(
//         address,
//         graceful_shutdown(Duration::from_secs(APP_CONFIG.graceful_wait_time)),
//     );
```

具体使用方式看qa-svc/src/main.rs

# tonic官方的Interceptor使用参考

- tonic中间件使用方式：https://github.com/hyperium/tonic/blob/master/examples/src/interceptor/server.rs
- tower方式定义的中间件参考：https://github.com/hyperium/tonic/blob/master/examples/src/tower/server.rs

tower中间件说明：

- 若需更强大的中间件，推荐采用tower方案。您可查看tower与tonic的使用示例。
- 此外，拦截器并非为服务添加日志记录功能的推荐方式。
- 对于此类需求，tower中间件更为合适，因为它还能对响应进行处理。例如，tower-http的Trace中间件开箱即支持gRPC。