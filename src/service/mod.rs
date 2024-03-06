use crate::crud;
use crate::crud::HandlerBuilder;
use serde_json::Value;

// Business logic
pub struct Service {
    pub(crate) host: String,
    pub(crate) api_key: String,
    pub(crate) username: String,
    pub(crate) path: String, // /api.php?_d=products for v1 or /api/2.0/products
    pub(crate) entity: String, // sub-entity e.g /api/2.0/categories/<id>/products
                             //  Break the path into path , entity and sub_entity methods
}

pub struct ServiceBuilder {
    pub(crate) host: String,
    pub(crate) api_key: String,
    pub(crate) username: String,
    pub(crate) path: String,   // /api.php?_d= for v1 or /api/2.0,
    pub(crate) entity: String, // last part of path
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
    pub fn path(mut self, path: &str) -> Self {
        self.path = path.to_string();
        self
    }

    pub fn build(self) -> Service {
        Service {
            host: self.host,
            api_key: self.api_key,
            username: self.username,
            path: self.path,
            entity: "".to_string(),
        }
    }
}

impl Service {
    pub fn new() -> ServiceBuilder {
        ServiceBuilder {
            host: "".to_string(),
            api_key: "".to_string(),
            username: "".to_string(),
            path: "".to_string(),
            entity: "".to_string(),
        }
    }

    fn set_handler_credentials(&self) -> HandlerBuilder {
        crud::Handler::new()
            .host(self.host.as_str())
            .username(self.username.as_str())
            .api_key(self.api_key.as_str())
    }

    pub async fn create(&self, data: Value) -> anyhow::Result<Value> {
        let handler = self
            .set_handler_credentials()
            .path(&(&self.path).to_string())
            .build();
        let rsp = handler.create(data).await?;
        Ok(rsp)
    }

    pub async fn get_all(&self) -> anyhow::Result<Value> {
        let handler = self
            .set_handler_credentials()
            .path(&(&self.path).to_string())
            .build();

        let rsp = handler.read().await?;
        Ok(rsp)
    }

    pub async fn get_by_id(&mut self, id: &str) -> anyhow::Result<Value> {
        let handler = self
            .set_handler_credentials()
            .path(&format!("{}/{}", &self.path, id))
            .build();

        let rsp = handler.read().await?;
        Ok(rsp)
    }

    pub async fn update_by_id(&self, id: &str, data: Value) -> anyhow::Result<Value> {
        let handler = self
            .set_handler_credentials()
            .path(&format!("{}/{}", &self.path, id))
            .build();

        let rsp = handler.update(data).await?;
        Ok(rsp)
    }

    pub async fn delete_by_id(&self, id: &str) -> anyhow::Result<Value> {
        let handler = self
            .set_handler_credentials()
            .path(&format!("{}/{}", &self.path, id))
            .build();

        let rsp = handler.delete().await?;
        Ok(rsp)
    }

    pub async fn get_all_entity(&mut self, id: &str, entity: &str) -> anyhow::Result<Value> {
        let handler = self
            .set_handler_credentials()
            .path(&format!("{}/{}/{}", &self.path, id, entity))
            .build();

        let rsp = handler.read().await?;
        Ok(rsp)
    }
}
