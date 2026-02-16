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
}

impl AboutMeDto {
    pub fn new(name: String, summary: String, quez_id: Vec<i32>,) -> Self {
        Self {name, summary, quez_id}
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
}