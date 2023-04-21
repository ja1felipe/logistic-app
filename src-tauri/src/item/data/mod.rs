use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateItemDTO {
    pub code: String,
    pub description: String,
    pub total_qtd: i32,
    pub category: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateItemDTO {
    pub description: String,
    pub total_qtd: i32,
    pub category: Option<String>,
}
