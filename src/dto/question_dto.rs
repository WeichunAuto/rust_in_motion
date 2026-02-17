use serde::{Deserialize, Serialize};

/**
 * 问题 与 回答 DTO
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QuestionDto {
    id: i32,
    quez: String,
    answer: Option<String>,
}

impl QuestionDto {
    pub fn new(id: i32, quez: String, answer: Option<String>) -> Self {
        Self { id, quez, answer }
    }

    pub fn get_quez(&self) -> String {
        self.quez.clone()
    }

    pub fn get_answer(&self) -> Option<String> {
        self.answer.clone()
    }
}
