use crate::prelude::*;
use crate::service::config::ServiceConfig;
use crate::types::Auth;
use serde::{Deserialize, Serialize};

pub struct AuthService {
    config: ServiceConfig<Authenticated>,
}

impl AuthService {
    pub fn with_config(service: ServiceConfig<Authenticated>) -> AuthService {
        Self { config: service }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateAuthRequest {
    pub email: String,
}

impl AuthService {
    pub async fn create(&self, data: CreateAuthRequest) -> anyhow::Result<Auth> {
        let handler = crate::handler::Handler::new()
            .host(&self.config.host())
            .username(self.config.auth().username())
            .api_key(self.config.auth().api_key())
            .path(self.config.resource.as_ref().unwrap().path())
            .build();
        let value = serde_json::to_value(data)?;
        let response_value = handler.create(value).await?;
        let response: Auth = serde_json::from_value(response_value)?;
        Ok(response)
    }
}
