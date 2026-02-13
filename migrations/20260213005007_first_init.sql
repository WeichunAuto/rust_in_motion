-- Add migration script here
-- Add migration script here
-- CREATE TYPE GENDER AS ENUM('Male', 'Female', 'Intersex');

-- create tab_menu table 首页菜单
CREATE TABLE IF NOT EXISTS tab_menu (
    id SERIAL PRIMARY KEY,
    menu_name VARCHAR(32) NOT NULL UNIQUE,
    create_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
    );

-- initial values for tab_menu; 初始化菜单值
INSERT INTO tab_menu (menu_name) VALUES ('About Me');
INSERT INTO tab_menu (menu_name) VALUES ('Projects');
INSERT INTO tab_menu (menu_name) VALUES ('Robotics');
INSERT INTO tab_menu (menu_name) VALUES ('Web');
INSERT INTO tab_menu (menu_name) VALUES ('Tools');


-- project_type for projects 项目类型
CREATE TABLE IF NOT EXISTS project_type (
    id SERIAL PRIMARY KEY,
    type_name VARCHAR(64) NOT NULL UNIQUE,
    create_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
    );

-- initial values for project_type; 初始化项目类型值
INSERT INTO project_type (type_name) VALUES ('Software Applications');
INSERT INTO project_type (type_name) VALUES ('Algorithms');
INSERT INTO project_type (type_name) VALUES ('ROS2');

-- ALTER TABLE USERS ADD COLUMN ws_id BIGINT REFERENCES workspace(id);

-- create project table 项目表
CREATE TABLE IF NOT EXISTS project (
    id SERIAL PRIMARY KEY,
    project_name VARCHAR(64) NOT NULL UNIQUE,
    introduction VARCHAR(512) NOT NULL,
    tags TEXT[] NOT NULL DEFAULT '{}',
    cover_image_url VARCHAR(128), -- 封面 URL路径
    type_id INT NOT NULL, 
    create_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,

    -- 外键约束
    CONSTRAINT fk_project_type
        FOREIGN KEY (type_id)
        REFERENCES project_type (id)
        ON DELETE CASCADE   
    );
-- 添加索引
CREATE INDEX idx_project_type_id ON project(type_id);


-- blog_category for blogs 博客文章分类
CREATE TABLE IF NOT EXISTS blog_category (
    id SERIAL PRIMARY KEY,
    category_name VARCHAR(64) NOT NULL UNIQUE,
    create_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
    );

-- initial values for blog_category; 初始化博客文章分类表
INSERT INTO blog_category (category_name) VALUES ('Robotics');
INSERT INTO blog_category (category_name) VALUES ('Web');


-- create blog table 博客文章表
CREATE TABLE IF NOT EXISTS blog (
    id SERIAL PRIMARY KEY,
    blog_title VARCHAR(254) NOT NULL,
    introduction TEXT NOT NULL,
    content TEXT NOT NULL, -- 存 markdown
    tags TEXT[] NOT NULL,
    cover_image_url VARCHAR(128), -- 封面 URL路径
    category_id INT NOT NULL, 
    create_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,

    -- 外键约束
    CONSTRAINT fk_blog_category
        FOREIGN KEY (category_id)
        REFERENCES blog_category (id)
        ON DELETE CASCADE   
    );
-- 添加索引
CREATE INDEX idx_blog_category_id ON blog(category_id);


-- create question table 问题
CREATE TABLE IF NOT EXISTS question (
    id SERIAL PRIMARY KEY,
    quez VARCHAR(128) NOT NULL UNIQUE,
    create_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
    );


-- create answer table 答案
CREATE TABLE IF NOT EXISTS answer (
    id SERIAL PRIMARY KEY,
    answer VARCHAR(1024) NOT NULL,
    quez_id INT NOT NULL, 
    create_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,

    -- 外键约束
    CONSTRAINT fk_answer_question
        FOREIGN KEY (quez_id)
        REFERENCES question(id)
        ON DELETE CASCADE
    );


-- create about_me table 关于我
CREATE TABLE IF NOT EXISTS about_me (
    id SERIAL PRIMARY KEY,
    name VARCHAR(64) NOT NULL UNIQUE,
    summary VARCHAR(1024) NOT NULL,
    quez_id INT[] NOT NULL,
    create_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
    );