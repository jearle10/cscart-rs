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
use crate::service::category;

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

    pub fn category (&self) -> category::Service {
        category::Service {
            host : self.host.to_string(),
            username: self.username.to_string(),
            api_key: self.api_key.to_string(),
            path : "/api/2.0/categories".to_string()
        }
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
    #[test]
    fn it_creates_api_client() {
        use super::*;
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
}
