-- Add migration script here
-- 给 tab_menu 表增加 display_order 字段
ALTER TABLE tab_menu
ADD COLUMN IF NOT EXISTS display_order INT DEFAULT 100;

update tab_menu set display_order=1 where id=1;
update tab_menu set display_order=2 where id=4;
update tab_menu set display_order=3 where id=3;
INSERT INTO tab_menu (menu_name, display_order) VALUES ('Product', 4);
update tab_menu set display_order=5 where id=5;

