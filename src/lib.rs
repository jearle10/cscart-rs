//! # About
//!
//! Cscart-rs is an sdk for using the cscart rest api
//!
//! This library is a work in process and so not all entities are supported for the v1
//! release
//!
//!  # Example
//! ```
//! use cscart_rs::prelude::*;
//! use cscart_rs::Client;
//! use serde_json::Value;
//! use anyhow;
//! use cscart_rs::GetAllCategoryResponse;
//!
//! async fn get_categories() -> anyhow::Result<GetAllCategoryResponse> {
//!     let client = Client::new()
//!         .host("http://my-ecommerce-site.com")
//!         .username("my-user-email@email.com")
//!         .api_key("user-api-key");
//!
//!     let categories = client
//!         .category()
//!         .get_all(GetAllOptions::default()).await;
//!
//!     categories
//! }
//! ```

mod handler;
mod request;
mod service;
mod types;
mod utils;

pub mod prelude {
    pub use crate::methods::*;
    pub use crate::request::Auth as ServiceAuth;
    pub use crate::service::state::*;
    pub use crate::service::*;
    pub use crate::types::*;
    pub use crate::utils::test_utils;
}

use auth_service::AuthService;
use block_service::BlockService;
use cart_service::CartService;
use category_service::CategoryService;
pub use category_service::{
    CreateCategoryResponse, GetAllCategoryResponse, GetAllProductsResponse, UpdateCategoryResponse,
};
use config::ServiceConfig;
use order_service::OrderService;
use prelude::*;
use product_service::ProductService;
use user_service::UserService;
use vendor_service::VendorService;

/// Configure an api client to perform requests
pub struct Client {
    username: String,
    api_key: String,
    host: String,
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

impl Client {
    pub fn new() -> Self {
        Client {
            username: "".to_string(),
            api_key: "".to_string(),
            host: "".to_string(),
        }
    }

    /// The cscart api user's email address
    pub fn username(mut self, username: &str) -> Self {
        self.username = username.to_string();
        self
    }

    pub fn api_key(mut self, api_key: &str) -> Self {
        self.api_key = api_key.to_string();
        self
    }

    pub fn host(mut self, host: &str) -> Self {
        self.host = host.to_string();
        self
    }

    pub fn auth(&self) -> AuthService {
        let config = ServiceConfig::with_resource(Resource::Auth)
            .host(self.host.as_str())
            .auth(ServiceAuth::from(&self.username, &self.api_key));
        AuthService::with_config(config)
    }

    pub fn block(&self) -> BlockService {
        let config = ServiceConfig::with_resource(Resource::Blocks)
            .host(self.host.as_str())
            .auth(ServiceAuth::from(&self.username, &self.api_key));
        BlockService::with_config(config)
    }

    pub fn cart(&self) -> CartService {
        let config = ServiceConfig::with_resource(Resource::Cart)
            .host(self.host.as_str())
            .auth(ServiceAuth::from(&self.username, &self.api_key));
        CartService::with_config(config)
    }

    pub fn call_request(&self) -> Service<Authenticated> {
        Service::with_resource(Resource::CallRequest)
            .host(self.host.as_str())
            .auth(ServiceAuth::from(&self.username, &self.api_key))
    }

    pub fn category(&self) -> CategoryService {
        let config = ServiceConfig::with_resource(Resource::Category)
            .host(self.host.as_str())
            .auth(ServiceAuth::from(&self.username, &self.api_key));
        CategoryService::with_config(config)
    }

    pub fn discussion(&self) -> Service<Authenticated> {
        Service::with_resource(Resource::Discussion)
            .host(self.host.as_str())
            .auth(ServiceAuth::from(&self.username, &self.api_key))
    }

    pub fn language(&self) -> Service<Authenticated> {
        Service::with_resource(Resource::Languages)
            .host(self.host.as_str())
            .auth(ServiceAuth::from(&self.username, &self.api_key))
    }

    pub fn langvars(&self) -> Service<Authenticated> {
        Service::with_resource(Resource::Langvars)
            .host(self.host.as_str())
            .auth(ServiceAuth::from(&self.username, &self.api_key))
    }

    pub fn order(&self) -> OrderService {
        let config = ServiceConfig::with_resource(Resource::Order)
            .host(self.host.as_str())
            .auth(ServiceAuth::from(&self.username, &self.api_key));
        OrderService::with_config(config)
    }

    pub fn page(&self) -> Service<Authenticated> {
        Service::with_resource(Resource::Pages)
            .host(self.host.as_str())
            .auth(ServiceAuth::from(&self.username, &self.api_key))
    }

    pub fn payment_method(&self) -> Service<Authenticated> {
        Service::with_resource(Resource::PaymentMethod)
            .host(self.host.as_str())
            .auth(ServiceAuth::from(&self.username, &self.api_key))
    }

    pub fn product(&self) -> ProductService {
        let config = ServiceConfig::with_resource(Resource::Product)
            .host(self.host.as_str())
            .auth(ServiceAuth::from(&self.username, &self.api_key));
        ProductService::with_config(config)
    }

    pub fn product_feature(&self) -> Service<Authenticated> {
        Service::with_resource(Resource::ProductFeature)
            .host(self.host.as_str())
            .auth(ServiceAuth::from(&self.username, &self.api_key))
    }

    pub fn product_option(&self) -> Service<Authenticated> {
        Service::with_resource(Resource::ProductOption)
            .host(self.host.as_str())
            .auth(ServiceAuth::from(&self.username, &self.api_key))
    }

    pub fn settings(&self) -> Service<Authenticated> {
        Service::with_resource(Resource::Settings)
            .host(self.host.as_str())
            .auth(ServiceAuth::from(&self.username, &self.api_key))
    }

    pub fn shipment(&self) -> Service<Authenticated> {
        Service::with_resource(Resource::Shipment)
            .host(self.host.as_str())
            .auth(ServiceAuth::from(&self.username, &self.api_key))
    }

    pub fn shipment_method(&self) -> Service<Authenticated> {
        Service::with_resource(Resource::ShipmentMethod)
            .host(self.host.as_str())
            .auth(ServiceAuth::from(&self.username, &self.api_key))
    }

    pub fn status(&self) -> Service<Authenticated> {
        Service::with_resource(Resource::Status)
            .host(self.host.as_str())
            .auth(ServiceAuth::from(&self.username, &self.api_key))
    }

    pub fn tax(&self) -> Service<Authenticated> {
        Service::with_resource(Resource::Tax)
            .host(self.host.as_str())
            .auth(ServiceAuth::from(&self.username, &self.api_key))
    }

    pub fn user(&self) -> UserService {
        let config = ServiceConfig::with_resource(Resource::User)
            .host(self.host.as_str())
            .auth(ServiceAuth::from(&self.username, &self.api_key));
        UserService::with_config(config)
    }

    pub fn user_group(&self) -> Service<Authenticated> {
        Service::with_resource(Resource::UserGroups)
            .host(self.host.as_str())
            .auth(ServiceAuth::from(&self.username, &self.api_key))
    }

    pub fn vendor(&self) -> VendorService {
        let config = ServiceConfig::with_resource(Resource::Vendor)
            .host(self.host.as_str())
            .auth(ServiceAuth::from(&self.username, &self.api_key));
        VendorService::with_config(config)
    }

    fn get_username(&self) -> &str {
        &self.username
    }

    fn get_api_key(&self) -> &str {
        &self.api_key
    }

    fn get_host(&self) -> &str {
        &self.host
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;
    #[test]
    fn it_creates_api_client() {
        dotenv().ok();

        let api_key = std::env::var("CSCART_API_KEY").expect("No api key found");

        let username = std::env::var("CSCART_USERNAME").expect("No username found");

        let host = std::env::var("CSCART_HOST").expect("No host found");

        let client = Client::new()
            .host(host.as_str())
            .username(username.as_str())
            .api_key(api_key.as_str());

        assert_eq!(client.get_username(), username);
        assert_eq!(client.get_api_key(), api_key);
        assert_eq!(client.get_host(), host);
    }
}
