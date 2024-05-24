use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    pub email: String,
    pub user_type: String,
    pub company_id: String,
    pub status: String,
    pub user_id: Option<String>,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub company: Option<String>,
    pub company_name: Option<String>,
    pub is_root: String,
    pub user_login: Option<String>,
    pub timestamp: Option<String>,
    pub password: Option<String>,
}
