use crate::handler;
use crate::handler::HandlerBuilder;
use crate::prelude::*;
use serde_json::Value;

// Business logic
pub struct Service {
    pub(crate) host: String,
    pub(crate) api_key: String,
    pub(crate) username: String,
    pub(crate) resource: Resource,
    pub(crate) entity: String, // sub-entity e.g /api/2.0/categories/<id>/products//  Break the path into path , entity and sub_entity methods
    pub(crate) params: Vec<(String, String)>,
}

pub struct ServiceBuilder {
    pub(crate) host: String,
    pub(crate) api_key: String,
    pub(crate) username: String,
    pub(crate) resource: Resource,
    pub(crate) entity: String, // last part of path
    pub(crate) params: Vec<(String, String)>,
}

impl ServiceBuilder {
    pub fn host(mut self, host: &str) -> Self {
        self.host = host.to_string();
        self
    }
    pub fn api_key(mut self, api_key: &str) -> Self {
        self.api_key = api_key.to_string();
        self
    }
    pub fn username(mut self, username: &str) -> Self {
        self.username = username.to_string();
        self
    }

    pub fn build(self) -> Service {
        Service {
            host: self.host,
            api_key: self.api_key,
            username: self.username,
            entity: "".to_string(),
            resource: self.resource,
            params: vec![],
        }
    }
}

impl Service {
    pub fn with_resource(resource: Resource) -> ServiceBuilder {
        ServiceBuilder {
            host: "".to_string(),
            api_key: "".to_string(),
            username: "".to_string(),
            entity: "".to_string(),
            params: vec![],
            resource,
        }
    }

    fn set_handler_credentials(&self) -> HandlerBuilder {
        handler::Handler::new()
            .host(self.host.as_str())
            .username(self.username.as_str())
            .api_key(self.api_key.as_str())
    }

    pub async fn create(&self, data: Value) -> anyhow::Result<Value> {
        let handler = self
            .set_handler_credentials()
            .path(self.resource.path().to_string())
            .build();
        let rsp = handler.create(data).await?;
        Ok(rsp)
    }

    pub async fn get_all(&self, options: GetAllOptions) -> anyhow::Result<Value> {
        // This method sometimes requires mandatory params to be provided (depending on resource e.g User requires user_type)
        let handler = self
            .set_handler_credentials()
            .path(self.resource.path().to_string())
            .set_query_params(options.params())
            .build();
        let rsp = handler.read().await?;
        Ok(rsp)
    }

    pub async fn get_by_id(&mut self, id: &str) -> anyhow::Result<Value> {
        let handler = self
            .set_handler_credentials()
            .path(format!("{}/{}", &self.resource.path().to_string(), id))
            .build();

        let rsp = handler.read().await?;
        Ok(rsp)
    }

    pub async fn update_by_id(&self, id: &str, data: Value) -> anyhow::Result<Value> {
        let handler = self
            .set_handler_credentials()
            .path(format!("{}/{}", &self.resource.path(), id))
            .build();

        let rsp = handler.update(data).await?;
        Ok(rsp)
    }

    pub async fn delete_by_id(&self, id: &str) -> anyhow::Result<Value> {
        let handler = self
            .set_handler_credentials()
            .path(format!("{}/{}", &self.resource.path(), id))
            .build();

        let rsp = handler.delete().await?;
        Ok(rsp)
    }

    pub async fn get_all_entity(&mut self, id: &str, entity: &str) -> anyhow::Result<Value> {
        let handler = self
            .set_handler_credentials()
            .path(format!("{}/{}/{}", &self.resource.path(), id, entity))
            .build();

        let rsp = handler.read().await?;
        Ok(rsp)
    }

    pub async fn send() -> anyhow::Result<Value> {
        Ok(Value::Null)
    }
}
