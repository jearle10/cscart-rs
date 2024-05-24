//! # About
//!
//! Cscart-rs is an sdk for making api request using the cscart rest api
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
//!
//! async fn get_categories() -> anyhow::Result<Value> {
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
    pub use crate::service::*;
    pub use crate::types::*;
}

use prelude::*;

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

    pub fn block(&self) -> Service {
        Service::with_resource(Resource::Blocks)
            .host(self.host.as_str())
            .api_key(self.api_key.as_str())
            .username(self.username.as_str())
            .build()
    }

    pub fn cart(&self) -> Service {
        Service::with_resource(Resource::Cart)
            .host(self.host.as_str())
            .api_key(self.api_key.as_str())
            .username(self.username.as_str())
            .build()
    }

    pub fn call_request(&self) -> Service {
        Service::with_resource(Resource::CallRequest)
            .host(self.host.as_str())
            .api_key(self.api_key.as_str())
            .username(self.username.as_str())
            .build()
    }

    pub fn category(&self) -> Service {
        Service::with_resource(Resource::Category)
            .host(self.host.as_str())
            .api_key(self.api_key.as_str())
            .username(self.username.as_str())
            .build()
    }

    pub fn discussion(&self) -> Service {
        Service::with_resource(Resource::Discussion)
            .host(self.host.as_str())
            .api_key(self.api_key.as_str())
            .username(self.username.as_str())
            .build()
    }

    pub fn language(&self) -> Service {
        Service::with_resource(Resource::Languages)
            .host(self.host.as_str())
            .api_key(self.api_key.as_str())
            .username(self.username.as_str())
            .build()
    }

    pub fn langvars(&self) -> Service {
        Service::with_resource(Resource::Langvars)
            .host(self.host.as_str())
            .api_key(self.api_key.as_str())
            .username(self.username.as_str())
            .build()
    }

    pub fn order(&self) -> Service {
        Service::with_resource(Resource::Order)
            .host(self.host.as_str())
            .api_key(self.api_key.as_str())
            .username(self.username.as_str())
            .build()
    }

    pub fn page(&self) -> Service {
        Service::with_resource(Resource::Pages)
            .host(self.host.as_str())
            .api_key(self.api_key.as_str())
            .username(self.username.as_str())
            .build()
    }

    pub fn payment_method(&self) -> Service {
        Service::with_resource(Resource::PaymentMethod)
            .host(self.host.as_str())
            .api_key(self.api_key.as_str())
            .username(self.username.as_str())
            .build()
    }

    pub fn product(&self) -> Service {
        Service::with_resource(Resource::Product)
            .host(self.host.as_str())
            .api_key(self.api_key.as_str())
            .username(self.username.as_str())
            .build()
    }

    pub fn product_feature(&self) -> Service {
        Service::with_resource(Resource::ProductFeature)
            .host(self.host.as_str())
            .api_key(self.api_key.as_str())
            .username(self.username.as_str())
            .build()
    }

    pub fn product_variation(&self) -> Service {
        Service::with_resource(Resource::ProductVariation)
            .host(self.host.as_str())
            .api_key(self.api_key.as_str())
            .username(self.username.as_str())
            .build()
    }

    pub fn product_variation_group(&self) -> Service {
        Service::with_resource(Resource::ProductVariationGroup)
            .host(self.host.as_str())
            .api_key(self.api_key.as_str())
            .username(self.username.as_str())
            .build()
    }

    pub fn product_option(&self) -> Service {
        Service::with_resource(Resource::ProductOption)
            .host(self.host.as_str())
            .api_key(self.api_key.as_str())
            .username(self.username.as_str())
            .build()
    }

    pub fn product_option_combination(&self) -> Service {
        Service::with_resource(Resource::ProductOptionCombination)
            .host(self.host.as_str())
            .api_key(self.api_key.as_str())
            .username(self.username.as_str())
            .build()
    }

    pub fn product_option_exceptions(&self) -> Service {
        Service::with_resource(Resource::ProductException)
            .host(self.host.as_str())
            .api_key(self.api_key.as_str())
            .username(self.username.as_str())
            .build()
    }

    pub fn settings(&self) -> Service {
        Service::with_resource(Resource::Settings)
            .host(self.host.as_str())
            .api_key(self.api_key.as_str())
            .username(self.username.as_str())
            .build()
    }

    pub fn shipment(&self) -> Service {
        Service::with_resource(Resource::Shipment)
            .host(self.host.as_str())
            .api_key(self.api_key.as_str())
            .username(self.username.as_str())
            .build()
    }

    pub fn shipment_method(&self) -> Service {
        Service::with_resource(Resource::ShipmentMethod)
            .host(self.host.as_str())
            .api_key(self.api_key.as_str())
            .username(self.username.as_str())
            .build()
    }

    pub fn status(&self) -> Service {
        Service::with_resource(Resource::Status)
            .host(self.host.as_str())
            .api_key(self.api_key.as_str())
            .username(self.username.as_str())
            .build()
    }

    pub fn tax(&self) -> Service {
        Service::with_resource(Resource::Tax)
            .host(self.host.as_str())
            .api_key(self.api_key.as_str())
            .username(self.username.as_str())
            .build()
    }

    pub fn user(&self) -> Service {
        Service::with_resource(Resource::User)
            .host(self.host.as_str())
            .api_key(self.api_key.as_str())
            .username(self.username.as_str())
            .build()
    }

    pub fn user_group(&self) -> Service {
        Service::with_resource(Resource::UserGroups)
            .host(self.host.as_str())
            .api_key(self.api_key.as_str())
            .username(self.username.as_str())
            .build()
    }

    pub fn vendor(&self) -> Service {
        Service::with_resource(Resource::Vendor)
            .host(self.host.as_str())
            .api_key(self.api_key.as_str())
            .username(self.username.as_str())
            .build()
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
