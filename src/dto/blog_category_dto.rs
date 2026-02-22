use serde::{Deserialize, Serialize};

/**
 * 博客类型 DTO
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BlogCategoryDto {
    id: i32,
    category_name: String,
}

impl BlogCategoryDto {
    pub fn new(id: i32, category_name: String) -> Self {
        Self { id, category_name }
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }

    pub fn get_category_name(&self) -> String {
        self.category_name.clone()
    }
}