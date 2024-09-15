-- 数据初始化操作
-- 添加用户
insert into members (openid,nick,level,points,used_points,expired_at,created_at)
values("a90e996387dd9b4864cc3ede82252574","lisi",0,200,0,"2024-12-08 10:00:00","2024-04-28 00:00:00"),
      ("e649ddfcb4f8fa398635f84c894148a2","zhangsan",0,100,0,"2024-11-10 10:00:00","2024-04-28 00:00:00");

-- 插入积分明细
insert into points_details (openid,points,action,reason,created_at)
values("a90e996387dd9b4864cc3ede82252574",200,"add","sys_init","2024-04-28 00:00:00"),
      ("e649ddfcb4f8fa398635f84c894148a2",100,"add","sys_init","2024-04-28 00:00:00");
