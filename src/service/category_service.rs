use crate::prelude::*;
use crate::service::config::ServiceConfig;
use crate::types::Category;
use crate::{
    impl_create_method, impl_delete_by_id_method, impl_get_all_entity_method, impl_get_all_method,
    impl_get_by_id_method, impl_update_by_id_method,
};
use serde::Deserialize;
use serde_json::Value;

pub struct CategoryService {
    config: ServiceConfig<Authenticated>,
}

impl CategoryService {
    pub fn with_config(service: ServiceConfig<Authenticated>) -> CategoryService {
        Self { config: service }
    }
}

#[derive(Deserialize, Debug)]
pub struct GetAllCategoryResponse {
    pub categories: Vec<Category>,
}

#[derive(Deserialize, Debug)]
pub struct GetAllProductsResponse {
    pub products: Vec<Product>,
}

#[derive(Deserialize, Debug)]
pub struct CreateCategoryResponse {
    pub category_id: i32,
}

#[derive(Deserialize, Debug)]
pub struct UpdateCategoryResponse {
    pub category_id: String,
}

impl_create_method!(CategoryService, CreateCategoryResponse);
impl_get_by_id_method!(CategoryService, Category);
impl_get_all_method!(CategoryService, GetAllCategoryResponse);
impl_update_by_id_method!(CategoryService, UpdateCategoryResponse);
impl_delete_by_id_method!(CategoryService, Value);
impl_get_all_entity_method!(CategoryService, GetAllProductsResponse);
