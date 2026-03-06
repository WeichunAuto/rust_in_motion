use serde::{Deserialize, Serialize};

/**
 * 博客DTO
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BlogResponsetDto {
    id: i32,
    blog_title: String,
    introduction: String,
    content: String,
    // Vec 格式的 tags, 每个元素就是一个tag，后端加载并返回到前端时使用
    vtags: Vec<String>,
    cover_image_url: Option<String>,
    category_id: i32,
    create_at: String,
    is_featured: bool,
}

impl BlogResponsetDto {
    pub fn new(
        id: i32,
        blog_title: String,
        introduction: String,
        content: String,
        // Vec 格式的 tags, 每个元素就是一个tag，后端加载并返回到前端时使用
        vtags: Vec<String>,
        cover_image_url: Option<String>,
        category_id: i32,
        create_at: String,
        is_featured: bool,
    ) -> Self {
        Self {
            id,
            blog_title,
            introduction,
            content,
            vtags,
            cover_image_url,
            category_id,
            create_at,
            is_featured,
        }
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }

    pub fn get_blog_title(&self) -> String {
        self.blog_title.clone()
    }

    pub fn get_introduction(&self) -> String {
        self.introduction.clone()
    }

    pub fn get_content(&self) -> String {
        self.content.clone()
    }

    pub fn get_vtags(&self) -> Vec<String> {
        self.vtags.clone()
    }

    pub fn get_cover_image_url(&self) -> Option<String> {
        self.cover_image_url.clone()
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

    pub fn set_is_featured(&mut self, target_featured: bool) {
        self.is_featured = target_featured;
    }
}

impl Default for BlogResponsetDto {
    fn default() -> Self {
        Self {
            id: Default::default(),
            blog_title: Default::default(),
            introduction: Default::default(),
            content: Default::default(),
            vtags: Default::default(),
            cover_image_url: Default::default(),
            category_id: Default::default(),
            create_at: Default::default(),
            is_featured: Default::default(),
        }
    }
}
