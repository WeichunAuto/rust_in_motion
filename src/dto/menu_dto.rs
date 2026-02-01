use serde::{Deserialize, Serialize};

/**
 * 菜单 DTO
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MenuDto {
    pub id: i32,
    pub menu_name: String,
}
