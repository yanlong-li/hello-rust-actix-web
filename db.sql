create database `actix_web_demo`;

use `actix_web_demo`;

create table `users`
(
    `id`         bigint unsigned not null auto_increment,
    `username`   varchar(255)    not null default '' comment '用户名',
    `password`   varchar(255)    not null default '' comment '密码',
    `created_at` timestamp       not null default current_timestamp,
    `updated_at` timestamp       not null default current_timestamp on update current_timestamp,
    primary key (`id`)
) comment ='用户表';