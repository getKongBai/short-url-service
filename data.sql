-- 用户表
CREATE TABLE users
(
    id int PRIMARY KEY
);

-- 短链接表
CREATE TABLE short_links
(
    id           varchar primary key,
    code         varchar unique null,
    short_url    varchar unique not null,
    original_url varchar        not null,
    user_id      int            not null references users (id)
);