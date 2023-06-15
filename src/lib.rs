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
//!
//! async fn get_categories() -> Result<Value , Box<dyn Error>> {
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

mod request;
mod crud;
mod service;

use dotenv::dotenv;

/// Configure an api client to perform requests
pub struct Client {
    username : String,
    api_key : String,
    host : String
}

impl Client {
    pub fn new() -> Self {
        Client {
            username: "".to_string(),
            api_key: "".to_string(),
            host: "".to_string()
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

    pub fn category (&self) -> service::Service {
        service::Service::new()
            .host(self.host.as_str())
            .api_key(self.api_key.as_str())
            .username(self.username.as_str())
            .path("/api/2.0/categories")
            .build()
    }

    pub fn product (&self) -> service::Service {
        service::Service::new()
            .host(self.host.as_str())
            .api_key(self.api_key.as_str())
            .username(self.username.as_str())
            .path("/api/2.0/products")
            .build()
    }

    pub fn user (&self) -> service::Service {
        service::Service::new()
            .host(self.host.as_str())
            .api_key(self.api_key.as_str())
            .username(self.username.as_str())
            .path("/api/2.0/users")
            .build()
    }

    pub fn cart (&self) -> service::Service {
        service::Service::new()
            .host(self.host.as_str())
            .api_key(self.api_key.as_str())
            .username(self.username.as_str())
            .path("/api/2.0/carts")
            .build()
    }

    pub fn vendor (&self) -> service::Service {
        service::Service::new()
            .host(self.host.as_str())
            .api_key(self.api_key.as_str())
            .username(self.username.as_str())
            .path("/api/2.0/vendors")
            .build()
    }

    pub fn payment_method (&self) -> service::Service {
        service::Service::new()
            .host(self.host.as_str())
            .api_key(self.api_key.as_str())
            .username(self.username.as_str())
            .path("/api/2.0/payments")
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
    #[test]
    fn it_creates_api_client() {

        dotenv().ok();

        let api_key = std::env::var("CSCART_API_KEY")
            .expect("No api key found");

        let username = std::env::var("CSCART_USERNAME")
            .expect("No username found");

        let host = std::env::var("CSCART_HOST")
            .expect("No host found");

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
             CART
         }

        impl Resource {

            fn to_string(&self ) -> String {
                match self {
                    Self::PRODUCT =>  String::from("products"),
                    Self::CART => String::from("carts")
                }
            }
        }

        let resource = Resource::PRODUCT.to_string();

        println!("{}" , resource)
    }
}
