-- 创建数据库
create database membership charset=utf8mb4;

-- 选择数据库
use membership;

-- 创建会员表
create table users(
    id bigint unsigned primary key not null auto_increment,
    openid varchar(32) not null comment '用户唯一标识uuid',
    name varchar(50) not null default '' comment '用户名字',
    level tinyint(1) unsigned not null default '0' comment '会员等级，0普通用户，1银卡用户，2金卡用户，3黑金用户，4白金用户，5钻石用户',
    nick varchar(50) not null default '' comment '用户昵称',
    phone varchar(50) not null default '' comment '用户手机号',
    age tinyint(3) unsigned not null default '0' comment '年龄',
    score bigint unsigned not null default '0' comment '用户积分'
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

-- 插入数据
insert into users(openid,name,level,nick,age,score) values
("0e0124838b60460da7816060e28de9a1","张三",0,"zhangsan",23,10),
("43ce2706907c4da481cd924587419bd0","李四",1,"lisi",28,80),
("a5a8a2c852db476a84dc51fdbe128dd4","小明",2,"xiaoming",32,120),
("c17bf3ecc31447409436236fcb55b9d5","小六",2,"xiaoliu",40,90);

-- 删除数据
delete from users where openid = "c17bf3ecc31447409436236fcb55b9d5";

-- 更新数据
update users set score = 130 where openid = "a5a8a2c852db476a84dc51fdbe128dd4";

-- 查询数据
select * from users where id >=1;

-- 添加唯一索引
alter table users add UNIQUE `uk_openid` (`openid`);

-- 添加普通索引
alter table users add index idx_nick (`nick`);

-- 删除索引
alter table users drop index idx_nick;

-- 查看表信息
show create table users\G;

-- 删除表
drop table users;

-- 删除数据库
drop database membership;
