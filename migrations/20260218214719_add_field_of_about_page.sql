-- Add migration script here

-- 给 about_me 表增加 about_page 字段
ALTER TABLE about_me ADD COLUMN about_page VARCHAR(1024);