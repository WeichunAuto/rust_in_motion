-- Add migration script here

-- 将 summary 的长度从 1024 改到 2048
ALTER TABLE about_me ALTER COLUMN summary TYPE VARCHAR(2048);
