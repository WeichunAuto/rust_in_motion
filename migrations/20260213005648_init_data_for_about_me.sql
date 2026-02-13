-- Add migration script here

-- 初始化 question 表数据.
insert into question(quez) values('Why do you blog?');
insert into question(quez) values('What is the meaning of "Rust in Motion"?');

-- 初始化 about_me 表数据
INSERT INTO about_me (name, summary, quez_id)
VALUES (
    'Bobby Wang',          -- 名字
    '',                   -- summary 为空字符串
    ARRAY[1, 2]           -- quez_id 数组
);