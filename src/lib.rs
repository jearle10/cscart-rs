mod request;
use dotenv::dotenv;

// Public Api
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
