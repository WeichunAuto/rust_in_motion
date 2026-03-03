-- Add migration script here
UPDATE blog_category set category_name = 'Product' where id = 2;
INSERT INTO blog_category (category_name) VALUES ('Web');
INSERT INTO blog_category (category_name) VALUES ('Tools');