-- 创建数据库 membership
create database membership charset=utf8mb4;

-- 选择数据库
use membership;

-- 创建会员表
create table users (
    id bigint unsigned primary key not null auto_increment,
    openid varchar(32) not null comment '用户唯一标识uuid',
    name varchar(50) not null default '' comment '用户名字',
    level tinyint(1) unsigned not null default '0' comment '会员等级，0普通用户，1银卡用户，2金卡用户，3黑金用户，4白金用户，5钻石用户',
    nick varchar(50) not null default '' comment '用户昵称',
    phone varchar(50) not null default '' comment '用户手机号',
    age tinyint(3) unsigned not null default '0' comment '年龄',
    score bigint unsigned not null default '0' comment '用户积分'
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
