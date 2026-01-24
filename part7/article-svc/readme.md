# article-svc

    文章阅读数更新服务

# usage

1. 先创建对应的数据库和插入数据，见`article.sql`文件。
2. 修改配置文件`app.yaml`
3. 执行如下命令，启动服务端

```shell
cargo run --bin article-svc
```

4. 启动job定时任务

```shell
cargo run --bin article-job
```

5. 执行如下命令访问文章详情接口

```shell
curl 'http://localhost:8080/api/article/1'
```

此时job就会执行文章阅读数增量更新
