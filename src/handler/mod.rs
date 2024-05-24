use crate::request;
use serde_json::{json, Value};

#[derive(Debug)]
pub struct Handler {
    username: String,
    api_key: String,
    host: String,
    path: String,
    query_params: Vec<(String, String)>,
    api: Option<request::Request>,
}

pub struct HandlerBuilder {
    username: String,
    api_key: String,
    host: String,
    path: String,
    query_params: Vec<(String, String)>,
    api: Option<request::Request>,
}

impl HandlerBuilder {
    pub(crate) fn username(mut self, username: &str) -> Self {
        self.username = username.to_string();
        self
    }

    pub(crate) fn api_key(mut self, api_key: &str) -> Self {
        self.api_key = api_key.to_string();
        self
    }

    pub(crate) fn host(mut self, host: &str) -> Self {
        self.host = host.to_string();
        self
    }

    pub(crate) fn path<T>(mut self, path: T) -> Self
    where
        T: Into<String>,
    {
        self.path = path.into();
        self
    }

    pub(crate) fn set_query_params(mut self, params: &[(String, String)]) -> Self {
        self.query_params = Vec::new();
        for param in params {
            self.query_params.push(param.clone());
        }
        self
    }

    pub(crate) fn build(self) -> Handler {
        Handler {
            username: self.username,
            api_key: self.api_key,
            host: self.host,
            path: self.path,
            query_params: self.query_params,
            api: self.api,
        }
    }
}

impl Handler {
    pub fn new() -> HandlerBuilder {
        HandlerBuilder {
            username: "".to_string(),
            api_key: "".to_string(),
            host: "".to_string(),
            path: "".to_string(),
            query_params: Vec::new(),
            api: None,
        }
    }

    pub(crate) async fn create(&self, body: Value) -> anyhow::Result<Value> {
        let request = request::Request::new()
            .host(&self.host)
            .path(&self.path)
            .username(&self.username)
            .api_key(&self.api_key)
            .build();

        let response: String = request.post(body).await?;

        let rsp = serde_json::from_str(&response)?;
        Ok(rsp)
    }

    pub(crate) async fn read(&self) -> anyhow::Result<Value> {
        let request = request::Request::new()
            .host(&self.host)
            .path(&self.path)
            .params(&self.query_params)
            .username(&self.username)
            .api_key(&self.api_key)
            .build();

        let response = request.get().await?;

        let rsp = serde_json::from_str(&response)?;
        Ok(rsp)
    }

    pub(crate) async fn update(&self, body: Value) -> anyhow::Result<Value> {
        let request = request::Request::new()
            .host(&self.host)
            .path(&self.path)
            .username(&self.username)
            .api_key(&self.api_key)
            .build();

        let response = request.put(body).await?;

        let rsp = serde_json::from_str(&response)?;
        Ok(rsp)
    }

    pub(crate) async fn delete(&self) -> anyhow::Result<Value> {
        let request = request::Request::new()
            .host(&self.host)
            .path(&self.path)
            .username(&self.username)
            .api_key(&self.api_key)
            .build();

        let response = request.delete().await?;

        let rsp = serde_json::from_str(&response).unwrap_or(json!(null));
        Ok(rsp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;
    use dotenv::dotenv;

    fn setup() -> Handler {
        dotenv().ok();
        let api_key = std::env::var("CSCART_API_KEY").expect("No api key found");

        let username = std::env::var("CSCART_USERNAME").expect("No username found");

        let host = std::env::var("CSCART_HOST").expect("No host found");

        Handler::new()
            .host(host.as_str())
            .username(username.as_str())
            .api_key(api_key.as_str())
            .path(Resource::Category.path().to_string())
            .build()
    }

    #[tokio::test]
    async fn it_creates_a_record() {
        let handler = setup();
        let data = serde_json::json!({"data" : 123});
        let response = handler.create(data).await;

        match response {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        }
    }

    #[tokio::test]
    async fn it_reads_a_record() {
        let handler = setup();
        let response = handler.read().await;

        match response {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        }
    }

    #[tokio::test]
    async fn it_updates_a_record() {
        let handler = setup();
        let data = serde_json::json!({"data" : 123});
        let response = handler.update(data).await;

        match response {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        }
    }

    #[tokio::test]
    async fn it_deletes_a_record() {
        let handler = setup();
        let response = handler.delete().await;

        match response {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        }
    }
}
