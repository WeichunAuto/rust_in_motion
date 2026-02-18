use serde::{Deserialize, Serialize};


/**
 * about me dto
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AboutMeDto {
    // 姓名
    name: String,
    // 简介
    summary: String,
    // 问题 id
    quez_id: Vec<i32>,
    // 关于 page
    about_page: Option<String>,
}

impl AboutMeDto {
    pub fn new(name: String, summary: String, quez_id: Vec<i32>, about_page: Option<String>) -> Self {
        Self {name, summary, quez_id, about_page}
    }
    
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_summary(&self) -> String {
        self.summary.clone()
    }

    pub fn get_quez_id(&self) -> Vec<i32> {
        self.quez_id.clone()
    }

    pub fn get_about_page(&self) -> Option<String> {
        self.about_page.clone()
    }
}