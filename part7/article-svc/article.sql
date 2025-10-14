-- 创建文章系统数据库
create database article_sys charset = utf8mb4;

-- 选择数据库
use article_sys;

-- 创建articles表
CREATE TABLE `articles` (
   `id` bigint unsigned NOT NULL AUTO_INCREMENT COMMENT '自增id',
   `title` varchar(1000) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL DEFAULT '' COMMENT '文章标题',
   `read_count` bigint unsigned NOT NULL DEFAULT '0' COMMENT '文章阅读数',
   `content` text CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL COMMENT '文章内容',
   `author` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL DEFAULT '' COMMENT '文章创建者',
   `created_at` datetime NOT NULL COMMENT '文章创建时间',
   `updated_at` datetime DEFAULT NULL COMMENT '文章更新时间',
   `is_deleted` tinyint unsigned NOT NULL DEFAULT '0' COMMENT '是否删除，1已删除，0正常',
   PRIMARY KEY (`id`),
   KEY `idx_created_at` (`created_at`),
   KEY `idx_author_created_at` (`author`,`created_at`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

-- 插入数据
insert into articles (title,content,author,created_at) values
   ("rust程序设计基本介绍","这是一篇讲解rust基础知识的文章","daheige","2024-12-08 10:10:00"),
   ("rust项目实战","rust项目实战的文章","daheige","2024-12-08 10:10:08");
