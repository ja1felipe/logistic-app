use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateUserInputDTO {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginUserInputDTO {
    pub email: String,
    pub password: String,
}
