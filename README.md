![Crates.io](https://img.shields.io/crates/l/cscart_rs/0.1.0)
![Crates.io](https://img.shields.io/crates/v/cscart_rs)
![Crates.io](https://img.shields.io/crates/d/cscart_rs)
![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/jearle10/cscart-rs/ci.yml)
### About
A rust based sdk for e-commerce platform CS cart.

NOTE: This is a work in progress!

### Getting started
```rust
use cscart_rs::Client;
use serde_json::Value;

async fn get_categories() -> Result<Value , Box<dyn Error>> {
    let client = Client::new()
        .host("http://my-ecommerce-site.com")
        .username("my-user-email@email.com")
        .api_key("user-api-key");
    
    let categories = client
        .category()
        .get_all().await;
    
    categories
}
```

### Features

| Entity                     |   Support   | 
| :---:                      |:-----------:|   
| cart                       |     Yes     |      
| call requests              |     Yes     |            
| categories                 |     Yes     |              
| discussions                |     No      |                    
| languages                  |     No      |                    
| langvars                   |     No      |                    
| orders                     |     Yes     |                    
| pages                      |     No      |                    
| payment methods            |     Yes     |                    
| products                   |     Yes     |                    
| product features           |     No      |                            
| product variations         |     No      |                    
| product variation group    |     No      |                    
| product options            |     No      |                    
| product option combination |     No      |                    
| product option exception   |     No      |                     
| settings                   | In progress |                    
| shipments                  | In progress |                     
| shipping methods           | In progress |                    
| statuses                   | In progress |                    
| stores                     |     No      |                    
| taxes                      | In progress |                    
| users                      |     Yes     |                    
| user groups                | In progress |                    
| vendors                    |     Yes     |