use crate::prelude::*;
use crate::service::config::ServiceConfig;
use crate::types::Vendor;
use crate::{
    impl_create_method, impl_delete_by_id_method, impl_get_all_entity_method, impl_get_all_method,
    impl_get_by_id_method, impl_update_by_id_method,
};
use serde::Deserialize;
use serde_json::Value;

pub struct VendorService {
    config: ServiceConfig<Authenticated>,
}

impl VendorService {
    pub fn with_config(service: ServiceConfig<Authenticated>) -> VendorService {
        Self { config: service }
    }
}

#[derive(Deserialize, Debug)]
pub struct GetAllVendorResponse {
    pub vendors: Vec<Vendor>,
}

#[derive(Deserialize, Debug)]
pub struct GetAllProductsResponse {
    pub vendors: Vec<Vendor>,
}

#[derive(Deserialize, Debug)]
pub struct CreateVendorResponse {
    pub company_id: i32,
    pub message: String,
}

#[derive(Deserialize, Debug)]
pub struct UpdateVendorResponse {
    pub store_id: String,
}

impl_create_method!(VendorService, CreateVendorResponse);
impl_get_by_id_method!(VendorService, Vendor);
impl_get_all_method!(VendorService, GetAllVendorResponse);
impl_update_by_id_method!(VendorService, UpdateVendorResponse);
impl_delete_by_id_method!(VendorService, Value);
impl_get_all_entity_method!(VendorService, GetAllProductsResponse);
