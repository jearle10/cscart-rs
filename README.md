![Crates.io](https://img.shields.io/crates/l/cscart_rs/0.1.0)
![Crates.io](https://img.shields.io/crates/v/cscart_rs)
![Crates.io](https://img.shields.io/crates/d/cscart_rs)
![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/jearle10/cscart-rs/ci.yml)
### About
A rust based sdk for e-commerce platform CS cart multivendor (https://www.cs-cart.com/)


### Getting started
```rust
 use cscart_rs::prelude::*;
 use cscart_rs::Client;
 use serde_json::Value;
 use anyhow;


 async fn get_categories() -> anyhow::Result<Vec<Category>> {
     let client = Client::new()
         .host("http://my-ecommerce-site.com")
         .username("my-user-email@email.com")
         .api_key("user-api-key");

     let categories = client
         .category()
         .get_all(GetAllOptions::default()).await;

    /*
      Requests and Response types are in progress for each entity.
      In the meantime there are types in the prelude..
      
      Check the CScart docs to know the request and response payloads
      if there isn't a type for the specific service you need

      Payload from the docs (https://docs.cs-cart.com/latest/developer_guide/api/entities/categories.html)
     */ 

    serde_json::from_value::<Vec<Category>>(categories)?;
 }
 ```

### Creating a category
```rust
 use cscart_rs::prelude::*;
 use cscart_rs::Client;
 use serde_json::Value;
 use anyhow;

async fn create_category() -> anyhow::Result<Value> {
    let client = Client::new()
        .host("http://my-ecommerce-site.com")
        .username("my-user-email@email.com")
        .api_key("user-api-key");

    let new_category = json!({
        "category" : "My new category",
        "company_id" : 1
    });
    
    let response = client
        .category()
        .create(new_category).await;
    
    println!("{:?}" , &response);
    
    response
}
```


### Features

All entities from the CScart documentation are supported<sup>*</sup> (https://docs.cs-cart.com/latest/developer_guide/api/index.html)

Typed API responses which are exported from the prelude, you can use these types optionally otherwise a `serde_json::Value` is returned

The following product services are not supported:
- product variation group                    
- product options                                
- product option combination                     
- product option exception 


### Contributing

Feel free to open a PR or open issues / bugs if you have features changes you want to discuss.
