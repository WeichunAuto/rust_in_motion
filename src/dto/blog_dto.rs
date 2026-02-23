use serde::{Deserialize, Deserializer, Serialize};

/**
 * 博客DTO
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BlogDto {
    id: Option<i32>,
    blog_title: String,
    introduction: String,
    content: Option<String>,
    tags: String,
    cover_image_url: Option<String>,
    cover_image_base64: Option<String>,
    category_id: i32,
}

impl BlogDto {
    pub fn new(
        id: Option<i32>,
        blog_title: String,
        introduction: String,
        content: Option<String>,
        tags: String,
        cover_image_url: Option<String>,
        cover_image_base64: Option<String>,
        category_id: i32,
    ) -> Self {
        Self {
            id,
            blog_title,
            introduction,
            content,
            tags,
            cover_image_url,
            cover_image_base64,
            category_id,
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

    pub fn get_tags(&self) -> String {
        self.tags.clone()
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
}
