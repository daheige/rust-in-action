# 项目结构体分层

项目分层主要分为config、entity、handlers、infras、routers等模块，每个模块所负责的事情如下所示：

- config：用于配置文件读取和mysql pool、pulsar client初始化。
  app.rs负责配置文件字段定义和配置读取。
  mysql.rs负责mysql pool连接池管理。
  xpulsar.rs负责pulsar消息队列配置信息字段定义和客户端连接。
- entity：用于积分系统用户信息、积分明细、积分信息Message基本信息等实体定义。
  member.rs负责会员基本信息字段定义。
  points_details.rs负责积分明细基本信息字段定义。
  points_message.rs负责积分Message消息字段定义和Message序列化和反序列化。
- handlers：用于http服务处理器函数定义。
  index.rs负责实现积分明细查询和积分增加/扣减业务逻辑处理。
- infras：用于积分系统基础设施层封装。
  config.rs负责配置文件读取。
  xmysql.rs负责mysql pool底层封装。
  xpulsar.rs负责pulsar客户端建立。
- routers：用于积分系统http服务路由配置。
  router.rs负责路由和http处理器函数的映射关系。
- main.rs：负责积分系统主入口文件。
- job.rs：负责积分系统消息异步消费，它是一个守护脚本。
