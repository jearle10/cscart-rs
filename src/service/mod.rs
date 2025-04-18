pub mod auth_service;
pub mod block_service;
pub mod cart_service;
pub mod category_service;
pub mod config;
pub mod order_service;
pub mod product_service;
pub mod user_service;
pub mod vendor_service;
#[macro_use]
pub mod methods;
pub mod state;

use crate::handler::HandlerBuilder;
use crate::handler::{self};
use crate::prelude::*;
use serde_json::Value;
use state::*;
use std::marker::PhantomData;

pub struct Service<S: ServiceState = Unauthenticated> {
    pub _marker: PhantomData<S>,
    pub(crate) host: Option<String>,
    pub(crate) auth: Option<ServiceAuth>,
    pub(crate) resource: Option<Resource>, // sub-entity e.g /api/2.0/categories/<id>/products//  Break the path into path , entity and sub_entity methods
    pub(crate) params: Option<Vec<(String, String)>>,
}

impl Service<Unauthenticated> {
    pub fn with_resource(resource: Resource) -> Service<Unauthenticated> {
        Service::<Unauthenticated> {
            _marker: PhantomData,
            host: None,
            auth: None,
            resource: Some(resource),
            params: None,
        }
    }
}

impl Service<Unauthenticated> {
    pub fn host(self, host: &str) -> Service<OnlyHost> {
        Service::<OnlyHost> {
            _marker: PhantomData,
            host: Some(host.into()),
            auth: None,
            resource: self.resource,
            params: None,
        }
    }

    pub fn auth(self, auth: ServiceAuth) -> Service<OnlyAuth> {
        Service::<OnlyAuth> {
            _marker: PhantomData,
            host: None,
            auth: Some(auth),
            resource: self.resource,
            params: None,
        }
    }
}

impl Service<OnlyHost> {
    pub fn auth(self, auth: ServiceAuth) -> Service<Authenticated> {
        Service::<Authenticated> {
            _marker: PhantomData,
            host: self.host,
            auth: Some(auth),
            resource: self.resource,
            params: None,
        }
    }
}

impl Service<OnlyAuth> {
    pub fn host(self, host: &str) -> Service<Authenticated> {
        Service::<Authenticated> {
            _marker: PhantomData,
            host: Some(host.into()),
            auth: self.auth,
            resource: self.resource,
            params: None,
        }
    }
}

impl Service<Authenticated> {
    fn set_handler_credentials(&self) -> HandlerBuilder {
        handler::Handler::new()
            .host(self.host.as_ref().unwrap().as_str())
            .username(self.auth.as_ref().unwrap().username())
            .api_key(self.auth.as_ref().unwrap().api_key())
    }

    pub async fn create(&self, data: Value) -> anyhow::Result<Value> {
        let handler = self
            .set_handler_credentials()
            .path(self.resource.as_ref().unwrap().path())
            .build();
        let rsp = handler.create(data).await?;
        Ok(rsp)
    }

    pub async fn get_all(&self, options: GetAllOptions) -> anyhow::Result<Value> {
        // This method sometimes requires mandatory params to be provided (depending on resource e.g User requires user_type)
        let handler = self
            .set_handler_credentials()
            .path(self.resource.as_ref().unwrap().path())
            .set_query_params(options.params())
            .build();
        let rsp = handler.read().await?;
        Ok(rsp)
    }

    pub async fn get_by_id(&mut self, id: &str) -> anyhow::Result<Value> {
        let handler = self
            .set_handler_credentials()
            .path(format!("{}/{}", self.resource.as_ref().unwrap().path(), id))
            .build();

        let rsp = handler.read().await?;
        Ok(rsp)
    }

    pub async fn update_by_id(&self, id: &str, data: Value) -> anyhow::Result<Value> {
        let handler = self
            .set_handler_credentials()
            .path(format!("{}/{}", self.resource.as_ref().unwrap().path(), id))
            .build();

        let rsp = handler.update(data).await?;
        Ok(rsp)
    }

    pub async fn delete_by_id(&self, id: &str) -> anyhow::Result<Value> {
        let handler = self
            .set_handler_credentials()
            .path(format!("{}/{}", self.resource.as_ref().unwrap().path(), id))
            .build();

        let rsp = handler.delete().await?;
        Ok(rsp)
    }

    pub async fn get_all_entity(&mut self, id: &str, entity: &str) -> anyhow::Result<Value> {
        let handler = self
            .set_handler_credentials()
            .path(format!(
                "{}/{}/{}",
                self.resource.as_ref().unwrap().path(),
                id,
                entity
            ))
            .build();

        let rsp = handler.read().await?;
        Ok(rsp)
    }
}
