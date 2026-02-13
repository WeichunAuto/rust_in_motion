-- Add migration script here
-- 给 question 表增加 answer 字段
ALTER TABLE question ADD COLUMN answer VARCHAR(1024);

-- 删除 answer 表
DROP TABLE IF EXISTS answer;