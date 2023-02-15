use crate::request;
use crate::crud;

use std::error::Error;
use serde_json::Value;

// Business logic
pub struct Service {
    pub(crate) host : String,
    pub(crate) api_key : String,
    pub(crate) username : String,
    pub(crate) path : String, // /api.php?_d= for v1 or /api/2.0
}


impl Service {

    pub async fn get_all(&self) -> Result<Value, Box<dyn Error>> {
        let handler = crud::Handler::new()
            .host(&self.host)
            .username(&self.username)
            .api_key(&self.api_key)
            .path(&format!("{}", &self.path))
            .build();

        handler.read().await
    }

    pub async fn get_by_id(&mut self , category_id : &str) -> Result<Value , Box<dyn Error>>{
        let handler = crud::Handler::new()
            .host(&self.host)
            .username(&self.username)
            .api_key(&self.api_key)
            .path(&format!("{}/{}", &self.path, category_id))
            .build();

        handler.read().await
    }

    pub async fn update_by_id(&self , category_id : &str, category : Value) -> Result<Value , Box<dyn Error>> {
        let handler = crud::Handler::new()
            .host(&self.host)
            .username(&self.username)
            .api_key(&self.api_key)
            .path(&format!("{}/{}", &self.path, category_id))
            .build();

        handler.update(category).await
    }

    pub async fn delete_by_id(&self, category_id : &str) -> Result<Value , Box<dyn Error>> {
        let handler = crud::Handler::new()
            .host(&self.host)
            .username(&self.username)
            .api_key(&self.api_key)
            .path(&format!("{}/{}", &self.path, category_id))
            .build();

        handler.delete().await
    }

    pub async fn get_all_products(&mut self, category_id : &str) -> Result<Value, Box<dyn Error>> {
        let handler = crud::Handler::new()
            .host(&self.host)
            .username(&self.username)
            .api_key(&self.api_key)
            .path(&format!("{}/{}/products", &self.path, category_id))
            .build();

        handler.read().await
    }
}