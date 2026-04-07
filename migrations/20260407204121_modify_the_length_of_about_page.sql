-- Add migration script here
-- 将表 about_me 中的字段 about_page 长度改成 2048.
ALTER TABLE about_me 
ALTER COLUMN about_page TYPE character varying(2048);