use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct Auth {
    pub token: String,
    pub refresh_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub email: String,
}
