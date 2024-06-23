use serde_json::Value;

#[derive(Debug, Clone)]
pub struct Auth {
    username: String,
    api_key: String,
}

impl Auth {
    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn api_key(&self) -> &str {
        &self.api_key
    }

    pub fn from<I>(username: I, api_key: I) -> Self
    where
        I: Into<String>,
    {
        Auth {
            username: username.into(),
            api_key: api_key.into(),
        }
    }
}

pub trait HttpClient {
    async fn get(url: String, auth: Auth, params: &[(String, String)]) -> anyhow::Result<String>;
    async fn put(url: String, auth: Auth, body: Value) -> anyhow::Result<String>;
    async fn post(url: String, auth: Auth, body: Value) -> anyhow::Result<String>;
    async fn delete(url: String, auth: Auth, body: Value) -> anyhow::Result<String>;
}

#[derive(Debug)]
pub struct Request {
    host: String,
    path: String,
    username: String,
    api_key: String,
    query_params: Vec<(String, String)>,
}

#[derive(Debug)]
pub struct RequestBuilder {
    host: String,
    path: String,
    username: String,
    api_key: String,
    query_params: Vec<(String, String)>,
}

impl RequestBuilder {
    pub fn host(mut self, host: &str) -> Self {
        self.host = host.to_string();
        self
    }

    pub fn path(mut self, path: &str) -> Self {
        self.path = path.to_string();
        self
    }

    pub fn username(mut self, username: &str) -> Self {
        self.username = username.to_string();
        self
    }

    pub fn api_key(mut self, api_key: &str) -> Self {
        self.api_key = api_key.to_string();
        self
    }

    pub fn params(mut self, params: &[(String, String)]) -> Self {
        for param in params {
            self.query_params.push(param.clone())
        }
        self
    }

    pub fn build(self) -> Request {
        Request {
            host: self.host,
            path: self.path,
            username: self.username,
            api_key: self.api_key,
            query_params: self.query_params,
        }
    }
}

#[allow(clippy::new_ret_no_self)]
impl Request {
    pub fn new() -> RequestBuilder {
        RequestBuilder {
            host: "".to_string(),
            path: "".to_string(),
            username: "".to_string(),
            query_params: vec![],
            api_key: "".to_string(),
        }
    }

    pub async fn get(&self) -> anyhow::Result<String> {
        let client = reqwest::Client::new();
        let endpoint = format!("{}{}", &self.host, &self.path);
        let url = reqwest::Url::parse_with_params(&endpoint, &self.query_params)?;
        let data = client
            .get(url)
            .basic_auth(self.username.as_str(), Some(self.api_key.as_str()))
            .query(&self.query_params)
            .send()
            .await?
            .text()
            .await?;

        Ok(data)
    }

    pub async fn put(&self, body: Value) -> anyhow::Result<String> {
        let client = reqwest::Client::new();
        let mut endpoint: String = String::from(&self.host);

        endpoint.push_str(&self.path);

        let data = client
            .put(endpoint)
            .basic_auth(self.username.as_str(), Some(self.api_key.as_str()))
            .query(&self.query_params)
            .json(&body)
            .send()
            .await?
            .text()
            .await?;

        Ok(data)
    }

    pub async fn post(&self, body: Value) -> anyhow::Result<String> {
        let client = reqwest::Client::new();
        let mut endpoint: String = String::from(&self.host);

        endpoint.push_str(&self.path);

        let data = client
            .post(endpoint)
            .basic_auth(self.username.as_str(), Some(self.api_key.as_str()))
            .query(&self.query_params)
            .json(&body)
            .send()
            .await?
            .text()
            .await?;
        Ok(data)
    }

    pub async fn delete(&self) -> anyhow::Result<String> {
        let client = reqwest::Client::new();
        let mut endpoint: String = String::from(&self.host);

        endpoint.push_str(&self.path);

        let data = client
            .delete(endpoint)
            .basic_auth(self.username.as_str(), Some(self.api_key.as_str()))
            .query(&self.query_params)
            .send()
            .await?
            .text()
            .await?;

        Ok(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;
    use serde_json::json;

    fn setup() -> Request {
        dotenv().ok();
        let api_key = std::env::var("CSCART_API_KEY").expect("No api key found");

        let username = std::env::var("CSCART_USERNAME").expect("No username found");

        let host = std::env::var("CSCART_HOST").expect("No host found");

        Request::new()
            .host(host.as_str())
            .path("/api/2.0/categories")
            .username(username.as_str())
            .api_key(api_key.as_str())
            .build()
    }

    #[tokio::test]
    async fn it_makes_a_get_request() {
        let client = setup();
        let response = client.get().await;

        match response {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        }
    }

    #[tokio::test]
    async fn it_makes_a_put_request() {
        let client = setup();
        let body = json!({"data" : "testing"});
        let response = client.put(body).await;

        match response {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        }
    }

    #[tokio::test]
    async fn it_makes_a_post_request() {
        let client = setup();
        let body = json!({"data" : "testing"});
        let response = client.post(body).await;

        match response {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        }
    }

    #[tokio::test]
    async fn it_makes_a_delete_request() {
        let client = setup();
        let response = client.delete().await;

        match response {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        }
    }
}
