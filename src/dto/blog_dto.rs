use reactive_stores::Store;
use serde::{Deserialize, Serialize};

/**
 * 博客DTO
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Store)]
pub struct BlogDto {
    id: Option<i32>,
    blog_title: String,
    introduction: String,
    content: Option<String>,
    // String 格式的 tags, 元素之间以,拼接，前端获取并保存到DB时使用
    tags: Option<String>,
    // Vec 格式的 tags, 每个元素就是一个tag，后端加载并返回到前端时使用
    vtags: Option<Vec<String>>,
    cover_image_url: Option<String>,
    cover_image_base64: Option<String>,
    category_id: i32,
    create_at: String,
    is_featured: bool,
}

impl BlogDto {
    pub fn new(
        id: Option<i32>,
        blog_title: String,
        introduction: String,
        content: Option<String>,
        // String 格式的 tags, 元素之间以,拼接，前端获取并保存到DB时使用
        tags: Option<String>,
        // Vec 格式的 tags, 每个元素就是一个tag，后端加载并返回到前端时使用
        vtags: Option<Vec<String>>,
        cover_image_url: Option<String>,
        cover_image_base64: Option<String>,
        category_id: i32,
        create_at: String,
        is_featured: bool,

    ) -> Self {
        Self {
            id,
            blog_title,
            introduction,
            content,
            tags,
            vtags,
            cover_image_url,
            cover_image_base64,
            category_id,
            create_at,
            is_featured,
        }
    }

    pub fn get_id(&self) -> Option<i32> {
        self.id
    }

    pub fn get_blog_title(&self) -> String {
        self.blog_title.clone()
    }

    pub fn get_introduction(&self) -> String {
        self.introduction.clone()
    }

    pub fn get_content(&self) -> Option<String> {
        self.content.clone()
    }

    pub fn get_tags(&self) -> Option<String> {
        self.tags.clone()
    }

    pub fn get_vtags(&self) -> Option<Vec<String>> {
        self.vtags.clone()
    }

    pub fn get_cover_image_url(&self) -> Option<String> {
        self.cover_image_url.clone()
    }

    pub fn get_cover_image_base64(&self) -> Option<String> {
        self.cover_image_base64.clone()
    }

    pub fn get_category_id(&self) -> i32 {
        self.category_id
    }

    pub fn get_create_at(&self) -> String {
        self.create_at.clone()
    }

    pub fn get_is_featured(&self) -> bool {
        self.is_featured
    }
}
