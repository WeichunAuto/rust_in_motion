-- Add migration script here

-- 给 blog 表增加 is_featured 字段
ALTER TABLE blog
ADD COLUMN is_featured BOOLEAN DEFAULT FALSE;
