use dotenv::dotenv;
use reqwest;
use serde_json::Value;

// Need to create a request interface to decouple from 3rd party http clients
#[derive(Debug)]
pub struct Request {
    host: String,
    path : String,
    username : String,
    api_key : String,
    query_params: Vec<(String , String)>
}

#[derive(Debug)]
pub struct RequestBuilder {
    host: String,
    path : String,
    username : String,
    api_key : String,
    query_params: Vec<(String , String)>
}

impl RequestBuilder {
    pub fn host (mut self , host :&str) -> Self {
        self.host = host.to_string();
        self
    }

    pub fn path(mut self, path : &str ) -> Self {
        self.path = path.to_string();
        self
    }

    pub fn username(mut self, username : &str ) -> Self {
        self.username = username.to_string();
        self
    }

    pub fn api_key(mut self, api_key : &str ) -> Self {
        self.api_key = api_key.to_string();
        self
    }

    pub fn params(mut self , params : Vec<(String , String)>) -> Self {
        for param in params {
            self.query_params.push(param)
        }
        self
    }

    pub fn build (self) -> Request {
        Request {
            host : self.host,
            path : self.path,
            username : self.username,
            api_key: self.api_key,
            query_params : self.query_params
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
            api_key: "".to_string()
        }
    }


    pub async fn get(&self) -> Result<String , reqwest::Error> {

        let client = reqwest::Client::new();
        let mut endpoint : String = String::from(&self.host);

        endpoint.push_str(&self.path);

        let data = client.get(endpoint)
            .basic_auth(
                self.username.as_str(),
                Some(self.api_key.as_str())
            )
            .query(&self.query_params)
            .send()
            .await?
            .text()
            .await;

        data
    }

    pub async fn put(&self, body : Value) -> Result<String , reqwest::Error> {
        let client = reqwest::Client::new();
        let mut endpoint : String = String::from(&self.host);

        endpoint.push_str(&self.path);

        let data = client.put(endpoint)
            .basic_auth(
                self.username.as_str(),
                Some(self.api_key.as_str())
            )
            .query(&self.query_params)
            .send()
            .await?
            .text()
            .await;

        data
    }

    pub async fn post(&self, body : Value) -> Result<String , reqwest::Error> {
        let client = reqwest::Client::new();
        let mut endpoint : String = String::from(&self.host);

        endpoint.push_str(&self.path);

        let data = client.post(endpoint)
            .basic_auth(
                self.username.as_str(),
                Some(self.api_key.as_str())
            )
            .query(&self.query_params)
            .send()
            .await?
            .text()
            .await;

        data
    }

    pub async fn delete(&self) -> Result<String , reqwest::Error> {
        let client = reqwest::Client::new();
        let mut endpoint : String = String::from(&self.host);

        endpoint.push_str(&self.path);

        let data = client.delete(endpoint)
            .basic_auth(
                self.username.as_str(),
                Some(self.api_key.as_str())
            )
            .query(&self.query_params)
            .send()
            .await?
            .text()
            .await;
        data
    }
}


#[cfg(test)]
mod tests {
    use serde_json::json;
    use super::*;

    fn setup() -> Request {
        dotenv().ok();
        let api_key = std::env::var("CSCART_API_KEY")
            .expect("No api key found");

        let username = std::env::var("CSCART_USERNAME")
            .expect("No username found");

        let host = std::env::var("CSCART_HOST")
            .expect("No host found");

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
            Err(_) => assert!(false)
        }
    }


    #[tokio::test]
    async fn it_makes_a_put_request() {

        let client = setup();
        let body = json!({"data" : "testing"});
        let response = client.put(body).await;

        match response {
            Ok(_) => assert!(true),
            Err(_) => assert!(false)
        }
    }

    #[tokio::test]
    async fn it_makes_a_post_request() {

        let client = setup();
        let body = json!({"data" : "testing"});
        let response = client.post(body).await;

        match response {
            Ok(_) => assert!(true),
            Err(_) => assert!(false)
        }
    }

    #[tokio::test]
    async fn it_makes_a_delete_request() {

        let client = setup();
        let response = client.delete().await;

        match response {
            Ok(_) => assert!(true),
            Err(_) => assert!(false)
        }
    }
}