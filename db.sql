drop database if exists `actix_web_demo`;
create database `actix_web_demo`;

use `actix_web_demo`;

drop table if exists `users`;
create table `users`
(
    `id`         bigint unsigned not null auto_increment,
    `username`   varchar(255)    not null default '' comment '用户名',
    `password`   varchar(255)    not null default '' comment '密码',
    `created_at` timestamp       not null default current_timestamp,
    `updated_at` timestamp       not null default current_timestamp on update current_timestamp,
    primary key (`id`)
) comment ='用户表';

alter table `users`
    add column deleted_at timestamp default null comment '删除时间';

insert into `users`(username, password, created_at, updated_at, deleted_at)
values ('test', 'test', current_timestamp, current_timestamp, null),
       ('test2', '123456', current_timestamp, current_timestamp, current_timestamp);