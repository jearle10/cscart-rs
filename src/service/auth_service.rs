use crate::impl_create_method;
use crate::prelude::*;
use crate::service::config::ServiceConfig;
use crate::types::Auth;

pub struct AuthService {
    config: ServiceConfig<Authenticated>,
}

impl AuthService {
    pub fn with_config(service: ServiceConfig<Authenticated>) -> AuthService {
        Self { config: service }
    }
}

impl_create_method!(AuthService, Auth);
