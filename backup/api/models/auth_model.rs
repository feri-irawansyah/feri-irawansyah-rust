use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub usernid: i32,
    pub email: String,
    pub fullname: String,
    pub disabled_login: bool,
    pub register_date: chrono::DateTime<chrono::Utc>,
    pub picture: Option<String>,
    pub comp_name: Option<String>,
    pub ip_address: Option<String>,
    pub app_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}