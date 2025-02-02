-- 创建积分系统数据库
create database points_sys charset=utf8mb4;

-- 选择数据库
use points_sys;

-- 会员表
CREATE TABLE `members` (
     `id` bigint unsigned NOT NULL AUTO_INCREMENT,
     `openid` varchar(32) COLLATE utf8mb4_general_ci NOT NULL COMMENT '用户唯一标识uuid',
     `phone` varchar(50) COLLATE utf8mb4_general_ci NOT NULL DEFAULT '' COMMENT '用户手机号',
     `nick` varchar(50) COLLATE utf8mb4_general_ci NOT NULL DEFAULT '' COMMENT '用户昵称',
     `level` tinyint unsigned NOT NULL DEFAULT '0' COMMENT '会员等级，0普通用户，1银卡用户，2金卡用户，3黑金用户，4白金用户，5钻石用户',
     `points` bigint unsigned NOT NULL DEFAULT '0' COMMENT '当前用户剩余积分',
     `used_points` bigint unsigned NOT NULL DEFAULT '0' COMMENT '当前用户已使用积分',
     `expired_at` datetime NOT NULL COMMENT '积分过期时间',
     `created_at` datetime NOT NULL COMMENT '创建时间',
     `updated_at` datetime DEFAULT NULL COMMENT '更新时间',
     PRIMARY KEY (`id`),
     UNIQUE KEY `uk_openid` (`openid`),
     KEY `idx_phone` (`phone`),
     KEY `idx_created_at` (`created_at`),
     KEY `idx_updated_at` (`updated_at`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT '会员表';

-- 积分明细表
CREATE TABLE `points_details` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `openid` varchar(32) COLLATE utf8mb4_general_ci NOT NULL COMMENT '用户唯一标识uuid',
  `points` bigint unsigned NOT NULL DEFAULT '0' COMMENT '使用或增加的积分数',
  `action` varchar(20) NOT NULL COMMENT '增加或扣减动作，add表示增加，sub表示扣除',
  `reason` varchar(100) NOT NULL COMMENT '操作理由',
  `created_at` datetime NOT NULL COMMENT '创建时间',
  PRIMARY KEY (`id`),
  KEY `idx_openid` (`openid`),
  KEY `idx_created_at` (`created_at`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT '积分明细表';

-- 数据初始化操作
-- 添加用户
insert into members (openid,nick,level,points,used_points,expired_at,created_at)
values("a90e996387dd9b4864cc3ede82252574","lisi",0,200,0,"2025-12-08 10:00:00","2024-12-08 00:00:00"),
      ("e649ddfcb4f8fa398635f84c894148a2","zhangsan",0,100,0,"2025-11-10 10:00:00","2024-12-08 00:00:00");

-- 插入积分明细
insert into points_details (openid,points,action,reason,created_at)
values("a90e996387dd9b4864cc3ede82252574",200,"add","sys_init","2024-12-08 20:00:00"),
      ("e649ddfcb4f8fa398635f84c894148a2",100,"add","sys_init","2024-12-08 20:01:00");
