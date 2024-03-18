//! # About
//!
//! Cscart-rs is an sdk for making api request using the cscart rest api
//!
//! This library is a work in process and so not all entities are supported for the v1
//! release
//!
//!  # Example
//! ```
//! use cscart_rs::Client;
//! use serde_json::Value;
//! use std::error::Error;
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
//!         .get_all().await;
//!
//!     categories
//! }
//! ```

mod crud;
mod request;
mod service;
pub mod types;

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

    pub fn block(&self) -> service::Service {
        service::Service::new()
            .host(self.host.as_str())
            .api_key(self.api_key.as_str())
            .username(self.username.as_str())
            .path("/api/2.0/blocks")
            .build()
    }

    pub fn cart(&self) -> service::Service {
        service::Service::new()
            .host(self.host.as_str())
            .api_key(self.api_key.as_str())
            .username(self.username.as_str())
            .path("/api/2.0/carts")
            .build()
    }

    pub fn call_request(&self) -> service::Service {
        service::Service::new()
            .host(self.host.as_str())
            .api_key(self.api_key.as_str())
            .username(self.username.as_str())
            .path("/api/2.0/call_requests")
            .build()
    }

    pub fn category(&self) -> service::Service {
        service::Service::new()
            .host(self.host.as_str())
            .api_key(self.api_key.as_str())
            .username(self.username.as_str())
            .path("/api/2.0/categories")
            .build()
    }

    pub fn discussion(&self) -> service::Service {
        service::Service::new()
            .host(self.host.as_str())
            .api_key(self.api_key.as_str())
            .username(self.username.as_str())
            .path("/api/2.0/discussions")
            .build()
    }

    pub fn language(&self) -> service::Service {
        service::Service::new()
            .host(self.host.as_str())
            .api_key(self.api_key.as_str())
            .username(self.username.as_str())
            .path("/api/2.0/languages")
            .build()
    }

    pub fn langvars(&self) -> service::Service {
        service::Service::new()
            .host(self.host.as_str())
            .api_key(self.api_key.as_str())
            .username(self.username.as_str())
            .path("/api/2.0/langvars")
            .build()
    }

    pub fn order(&self) -> service::Service {
        service::Service::new()
            .host(self.host.as_str())
            .api_key(self.api_key.as_str())
            .username(self.username.as_str())
            .path("/api/2.0/orders")
            .build()
    }

    pub fn page(&self) -> service::Service {
        service::Service::new()
            .host(self.host.as_str())
            .api_key(self.api_key.as_str())
            .username(self.username.as_str())
            .path("/api/2.0/pages")
            .build()
    }

    pub fn payment_method(&self) -> service::Service {
        service::Service::new()
            .host(self.host.as_str())
            .api_key(self.api_key.as_str())
            .username(self.username.as_str())
            .path("/api/2.0/payments")
            .build()
    }

    pub fn product(&self) -> service::Service {
        service::Service::new()
            .host(self.host.as_str())
            .api_key(self.api_key.as_str())
            .username(self.username.as_str())
            .path("/api/2.0/products")
            .build()
    }

    pub fn product_feature(&self) -> service::Service {
        service::Service::new()
            .host(self.host.as_str())
            .api_key(self.api_key.as_str())
            .username(self.username.as_str())
            .path("/api/2.0/features")
            .build()
    }

    pub fn product_variation(&self) -> service::Service {
        service::Service::new()
            .host(self.host.as_str())
            .api_key(self.api_key.as_str())
            .username(self.username.as_str())
            .path("/api/2.0/product_variation")
            .build()
    }

    pub fn product_variation_group(&self) -> service::Service {
        service::Service::new()
            .host(self.host.as_str())
            .api_key(self.api_key.as_str())
            .username(self.username.as_str())
            .path("/api/2.0/product_variation_groups")
            .build()
    }

    pub fn product_option(&self) -> service::Service {
        service::Service::new()
            .host(self.host.as_str())
            .api_key(self.api_key.as_str())
            .username(self.username.as_str())
            .path("/api/2.0/options")
            .build()
    }

    pub fn product_option_combination(&self) -> service::Service {
        service::Service::new()
            .host(self.host.as_str())
            .api_key(self.api_key.as_str())
            .username(self.username.as_str())
            .path("/api/2.0/combinations")
            .build()
    }

    pub fn product_option_exceptions(&self) -> service::Service {
        service::Service::new()
            .host(self.host.as_str())
            .api_key(self.api_key.as_str())
            .username(self.username.as_str())
            .path("/api/2.0/exceptions")
            .build()
    }

    pub fn settings(&self) -> service::Service {
        service::Service::new()
            .host(self.host.as_str())
            .api_key(self.api_key.as_str())
            .username(self.username.as_str())
            .path("/api/2.0/settings")
            .build()
    }

    pub fn shipment(&self) -> service::Service {
        service::Service::new()
            .host(self.host.as_str())
            .api_key(self.api_key.as_str())
            .username(self.username.as_str())
            .path("/api/2.0/shipments")
            .build()
    }

    pub fn shipment_method(&self) -> service::Service {
        service::Service::new()
            .host(self.host.as_str())
            .api_key(self.api_key.as_str())
            .username(self.username.as_str())
            .path("/api/2.0/shippings")
            .build()
    }

    pub fn status(&self) -> service::Service {
        service::Service::new()
            .host(self.host.as_str())
            .api_key(self.api_key.as_str())
            .username(self.username.as_str())
            .path("/api/2.0/statuses")
            .build()
    }

    pub fn tax(&self) -> service::Service {
        service::Service::new()
            .host(self.host.as_str())
            .api_key(self.api_key.as_str())
            .username(self.username.as_str())
            .path("/api/2.0/taxes")
            .build()
    }

    pub fn user(&self) -> service::Service {
        service::Service::new()
            .host(self.host.as_str())
            .api_key(self.api_key.as_str())
            .username(self.username.as_str())
            .path("/api/2.0/users")
            .build()
    }

    pub fn user_group(&self) -> service::Service {
        service::Service::new()
            .host(self.host.as_str())
            .api_key(self.api_key.as_str())
            .username(self.username.as_str())
            .path("/api/2.0/usergroups")
            .build()
    }

    pub fn vendor(&self) -> service::Service {
        service::Service::new()
            .host(self.host.as_str())
            .api_key(self.api_key.as_str())
            .username(self.username.as_str())
            .path("/api/2.0/vendors")
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

    #[test]
    fn resource_enum() {
        enum Resource {
            PRODUCT,
            CART,
        }

        impl Resource {
            fn to_string(&self) -> String {
                match self {
                    Self::PRODUCT => String::from("products"),
                    Self::CART => String::from("carts"),
                }
            }
        }

        let resource = Resource::PRODUCT.to_string();

        println!("{}", resource)
    }
}
