use crate::prelude::*;
use crate::service::config::ServiceConfig;
use crate::types::Product;
use crate::{
    impl_create_method, impl_delete_by_id_method, impl_get_all_method, impl_get_by_id_method,
    impl_update_by_id_method,
};
use serde::Deserialize;
use serde_json::Value;

pub struct ProductService {
    config: ServiceConfig<Authenticated>,
}

impl ProductService {
    pub fn with_config(service: ServiceConfig<Authenticated>) -> ProductService {
        Self { config: service }
    }
}

#[derive(Deserialize, Debug)]
pub struct GetAllProductResponse {
    pub products: Vec<Product>,
}

#[derive(Deserialize, Debug)]
pub struct CreateProductResponse {
    pub product_id: i32,
}

#[derive(Deserialize, Debug)]
pub struct UpdateProductResponse {
    pub product_id: i32,
}

#[derive(Deserialize, Debug)]
pub struct DeleteProductResponse {
    pub product_id: String,
}

impl_create_method!(ProductService, CreateProductResponse);
impl_get_by_id_method!(ProductService, Product);
impl_get_all_method!(ProductService, GetAllProductResponse);
impl_update_by_id_method!(ProductService, UpdateProductResponse);
impl_delete_by_id_method!(ProductService, DeleteProductResponse);
