![Crates.io](https://img.shields.io/crates/l/cscart_rs/0.1.0)
![Crates.io](https://img.shields.io/crates/v/cscart_rs)
![Crates.io](https://img.shields.io/crates/d/cscart_rs)
![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/jearle10/cscart-rs/ci.yml)
### About
A rust based sdk for e-commerce platform CS cart multivendor (https://www.cs-cart.com/)

NOTE: This is not yet stable and is a work in progress!

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
    
    println!("{:?}" , &categories);
    
    categories
}
```

### Creating a category
```rust
use cscart_rs::Client;
use serde_json::Value;

async fn create_category() -> Result<Value , Box<dyn Error>> {
    let client = Client::new()
        .host("http://my-ecommerce-site.com")
        .username("my-user-email@email.com")
        .api_key("user-api-key");
    
    /*
      Requests and Response types are in progress for each entity...
      In the meantime check the CScart docs to know the request and response payloads
     */
    
    // Payload from the docs (https://docs.cs-cart.com/latest/developer_guide/api/entities/categories.html) 
    let new_category = json!({
        "category" : "My new category",
        "company_id" : 1
    });
    
    let category = client
        .category()
        .create(new_category).await;
    
    println!("{:?}" , &category);
    
    category
}
```


### Features

All entities from the CScart documentation are supported (https://docs.cs-cart.com/latest/developer_guide/api/index.html)

Entities that have been tested are tracked below (work in progress, so still may be some bugs untested services...)

| Entity                     |   Tested    | 
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
