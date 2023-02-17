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

| Entity                     |        Support       | 
| :---:                      |         :-----:      |   
| cart                       |   :white_check_mark: |      
| call requests              |   :clock9:           |            
| categories                 |   :white_check_mark: |              
| discussions                |         :x:          |                    
| languages                  |         :x:          |                    
| langvars                   |         :x:          |                    
| orders                     |       :clock9:       |                    
| pages                      |         :x:          |                    
| payment methods            |   :white_check_mark: |                    
| products                   |   :white_check_mark: |                    
| product features           |         :x:          |                            
| product variations         |         :x:          |                    
| product variation group    |         :x:          |                    
| product options            |         :x:          |                    
| product option combination |         :x:          |                    
| product option exception   |         :x:          |                     
| settings                   |       :clock9:       |                    
| shipments                  |       :clock9:       |                     
| shipping methods           |       :clock9:       |                    
| statuses                   |       :clock9:       |                    
| stores                     |         :x:          |                    
| taxes                      |       :clock9:       |                    
| users                      |   :white_check_mark: |                    
| user groups                |        :clock9:      |                    
| vendors                    |   :white_check_mark: |                    


#### More functionality coming soon :clock9: