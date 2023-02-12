### About
A rust based sdk for e-commerce platform CS cart

### Getting started
```rust
use cscart_rs::Client;

let client = Client::new()
    .host("http://my-ecommerce-site.com")
    .username("my-user-email@email.com")
    .api_key("user-api-key");
```