use std::error::Error;
use crate::request;
use serde_json::Value;


type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Handler {
    username : String,
    api_key : String,
    host : String,
    path : String,
}

pub struct HandlerBuilder {
    username : String,
    api_key : String,
    host : String,
    path : String,
}

impl HandlerBuilder {
    pub (crate) fn username (mut self, username : &str) -> Self {
        self.username = username.to_string();
        self
    }

    pub (crate) fn api_key (mut self, api_key : &str) -> Self {
        self.api_key = api_key.to_string();
        self
    }

    pub (crate) fn host (mut self, host : &str) -> Self {
        self.host = host.to_string();
        self
    }

    pub (crate) fn path (mut self, path : &str) -> Self {
        self.path = path.to_string();
        self
    }

    pub (crate) fn build(self) -> Handler {
        Handler {
            username: self.username,
            api_key: self.api_key,
            host: self.host,
            path: self.path
        }
    }
}

impl Handler {

    pub fn new () -> HandlerBuilder {
        HandlerBuilder {
            username: "".to_string(),
            api_key: "".to_string(),
            host: "".to_string(),
            path : "".to_string()
        }
    }


    pub(crate) async fn create(&self, body : Value) -> Result<Value>{
        let request = request::Request::new()
            .host(&self.host)
            .path(&self.path)
            .username(&self.username)
            .api_key(&self.api_key)
            .build();

        let response = request.post(body).await?;

        match serde_json::from_str(&response) {
            Ok(data) => Ok(data),
            Err(e) => Err(Box::new(e))
        }
    }

    pub(crate) async fn read(&self) -> Result<Value>{
        let request = request::Request::new()
            .host(&self.host)
            .path(&self.path)
            .username(&self.username)
            .api_key(&self.api_key)
            .build();

        let response = request.get().await?;

        match serde_json::from_str(&response) {
            Ok(data) => Ok(data),
            Err(e) => Err(Box::new(e))
        }
    }

    pub(crate) async fn update(&self, body: Value) -> Result<Value>{
        let request = request::Request::new()
            .host(&self.host)
            .path(&self.path)
            .username(&self.username)
            .api_key(&self.api_key)
            .build();

        let response = request.put(body).await?;

        match serde_json::from_str(&response) {
            Ok(data) => Ok(data),
            Err(e) => Err(Box::new(e))
        }
    }

    pub(crate) async fn delete(&self) -> Result<Value>{
        let request = request::Request::new()
            .host(&self.host)
            .path(&self.path)
            .username(&self.username)
            .api_key(&self.api_key)
            .build();

        let response = request.delete().await?;

        match serde_json::from_str(&response) {
            Ok(data) => Ok(data),
            Err(e) => Err(Box::new(e))
        }
    }
}


#[cfg(test)]
mod tests {
    use dotenv::dotenv;
    use super::*;

    fn setup() -> Handler  {
        dotenv().ok();
        let api_key = std::env::var("CSCART_API_KEY")
            .expect("No api key found");

        let username = std::env::var("CSCART_USERNAME")
            .expect("No username found");

        let host = std::env::var("CSCART_HOST")
            .expect("No host found");

        Handler::new()
            .host(host.as_str())
            .path("/api/2.0/categories")
            .username(username.as_str())
            .api_key(api_key.as_str())
            .build()
    }

    #[tokio::test]
    async fn it_creates_a_record() {
        let handler = setup();
        let data = serde_json::json!({"data" : 123});
        let response = handler.create(data).await;

        match response {
            Ok(_) => assert!(true),
            Err(_) => assert!(false)
        }
    }

    #[tokio::test]
    async fn it_reads_a_record() {
        let handler = setup();
        let response = handler.read().await;

        match response {
            Ok(_) => assert!(true),
            Err(_) => assert!(false)
        }
    }

    #[tokio::test]
    async fn it_updates_a_record() {
        let handler = setup();
        let data = serde_json::json!({"data" : 123});
        let response = handler.update(data).await;

        match response {
            Ok(_) => assert!(true),
            Err(_) => assert!(false)
        }
    }

    #[tokio::test]
    async fn it_deletes_a_record() {
        let handler = setup();
        let response = handler.delete().await;

        match response {
            Ok(_) => assert!(true),
            Err(_) => assert!(false)
        }
    }
}