use crate::prelude::*;
use serde_json::Value;
use std::future::Future;

pub trait CreateResource {
    type Response;
    fn create(
        &self,
        data: serde_json::Value,
    ) -> impl Future<Output = anyhow::Result<Self::Response>>;
}

pub trait GetAllResource {
    type Response;
    fn get_all(
        &self,
        options: GetAllOptions,
    ) -> impl Future<Output = anyhow::Result<Self::Response>>;
}

pub trait GetResourceById {
    type Response;
    fn get_by_id<T: Into<String>>(
        &self,
        id: T,
    ) -> impl Future<Output = anyhow::Result<Self::Response>>;
}

pub trait UpdateResource {
    type Response;
    fn update_by_id<T: Into<String>>(
        &self,
        id: T,
        data: Value,
    ) -> impl Future<Output = anyhow::Result<Self::Response>>;
}

pub trait DeleteResource {
    type Response;
    fn delete_by_id<T: Into<String>>(
        &self,
        id: T,
    ) -> impl Future<Output = anyhow::Result<Self::Response>>;
}

pub trait GetAllSubResource {
    type Response;
    fn get_all_sub_entity<T: Into<String>>(
        &self,
        id: T,
        entity: T,
    ) -> impl Future<Output = anyhow::Result<Self::Response>>;
}

#[macro_export]
macro_rules! impl_create_method {
    ($service:ident, $return_type:ident) => {
        impl CreateResource for $service {
            type Response = $return_type;
            async fn create(&self, data: serde_json::Value) -> anyhow::Result<Self::Response> {
                let handler = $crate::handler::Handler::new()
                    .host(&self.config.host())
                    .username(self.config.auth().username())
                    .api_key(self.config.auth().api_key())
                    .path(self.config.resource.as_ref().unwrap().path())
                    .build();

                let response_value = handler.create(data).await?;
                let response: $return_type = serde_json::from_value(response_value)?;
                Ok(response)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_get_by_id_method {
    ($service:ident, $return_type:ident) => {
        impl GetResourceById for $service {
            type Response = $return_type;
            async fn get_by_id<T: Into<String>>(&self, id: T) -> anyhow::Result<Self::Response> {
                let handler = $crate::handler::Handler::new()
                    .host(&self.config.host())
                    .username(self.config.auth().username())
                    .api_key(self.config.auth().api_key())
                    .path(format!(
                        "{}/{}",
                        self.config.resource.as_ref().unwrap().path(),
                        id.into()
                    ))
                    .build();

                let response_value = handler.read().await?;
                let response: $return_type = serde_json::from_value(response_value)?;
                Ok(response)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_get_all_method {
    ($service:ident, $return_type:ident) => {
        impl GetAllResource for $service {
            type Response = $return_type;
            async fn get_all(&self, options: GetAllOptions) -> anyhow::Result<Self::Response> {
                let handler = $crate::handler::Handler::new()
                    .host(&self.config.host())
                    .username(self.config.auth().username())
                    .api_key(self.config.auth().api_key())
                    .path(self.config.resource.as_ref().unwrap().path())
                    .set_query_params(options.params())
                    .build();

                let response_value = handler.read().await?;
                let response: $return_type = serde_json::from_value(response_value)?;
                Ok(response)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_update_by_id_method {
    ($service:ident, $return_type:ident) => {
        impl UpdateResource for $service {
            type Response = $return_type;
            async fn update_by_id<T: Into<String>>(
                &self,
                id: T,
                data: Value,
            ) -> anyhow::Result<Self::Response> {
                let handler = Handler::new()
                    .host(&self.config.host())
                    .username(self.config.auth().username())
                    .api_key(self.config.auth().api_key())
                    .path(format!(
                        "{}/{}",
                        self.config.resource.as_ref().unwrap().path(),
                        id.into()
                    ))
                    .build();

                let response_value = handler.update(data).await?;
                let response: $return_type = serde_json::from_value(response_value)?;
                Ok(response)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_delete_by_id_method {
    ($service:ident, $return_type:ident) => {
        use $crate::handler::Handler;
        impl DeleteResource for $service {
            type Response = $return_type;
            async fn delete_by_id<T: Into<String>>(&self, id: T) -> anyhow::Result<Self::Response> {
                let handler = Handler::new()
                    .host(&self.config.host())
                    .username(self.config.auth().username())
                    .api_key(self.config.auth().api_key())
                    .path(format!(
                        "{}/{}",
                        self.config.resource.as_ref().unwrap().path(),
                        id.into()
                    ))
                    .build();

                let response_value = handler.delete().await?;
                let response: $return_type = serde_json::from_value(response_value)?;
                Ok(response)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_get_all_entity_method {
    ($service:ident, $return_type:ident) => {
        impl GetAllSubResource for $service {
            type Response = $return_type;
            async fn get_all_sub_entity<T: Into<String>>(
                &self,
                id: T,
                entity: T,
            ) -> anyhow::Result<Self::Response> {
                let handler = $crate::handler::Handler::new()
                    .host(&self.config.host())
                    .username(self.config.auth().username())
                    .api_key(self.config.auth().api_key())
                    .path(format!(
                        "{}/{}/{}",
                        self.config.resource.as_ref().unwrap().path(),
                        id.into(),
                        entity.into()
                    ))
                    .build();

                let response_value = handler.read().await?;
                dbg!(&response_value);
                let response: $return_type = serde_json::from_value(response_value)?;
                Ok(response)
            }
        }
    };
}
