use crate::prelude::*;
use crate::service::config::ServiceConfig;

use crate::{
    impl_create_method, impl_delete_by_id_method, impl_get_all_method, impl_get_by_id_method,
    impl_update_by_id_method,
};
use serde::Deserialize;
use serde_json::Value;

use crate::types::User;

pub struct UserService {
    config: ServiceConfig<Authenticated>,
}

impl UserService {
    pub fn with_config(service: ServiceConfig<Authenticated>) -> UserService {
        Self { config: service }
    }
}

#[derive(Deserialize, Debug)]
pub struct CreateUserResponse {
    pub user_id: i32,
    pub profile_id: String,
}

#[derive(Deserialize, Debug)]
pub struct UpdateUserResponse {
    pub user_id: i32,
    pub profile_id: String,
}

#[derive(Deserialize, Debug)]
pub struct GetAllUsersResponse {
    pub users: Vec<User>,
}

impl_create_method!(UserService, CreateUserResponse);
impl_get_by_id_method!(UserService, User);
impl_get_all_method!(UserService, GetAllUsersResponse);
impl_update_by_id_method!(UserService, Value);
impl_delete_by_id_method!(UserService, Value);
