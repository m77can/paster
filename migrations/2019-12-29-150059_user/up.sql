-- Your SQL goes here
create table `user`
(
    `id`            bigint(20),
    `username`      varchar(100) not null,
    `password_hash` varchar(100) not null,
    `salt`          varchar(100) not null,
    `created_at`    timestamp    not null default current_timestamp(),
    `updated_at`    timestamp             DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    primary key (`id`),
    unique key (`username`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4;