use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateCategoryDTO {
    pub code: String,
    pub description: String,
}
