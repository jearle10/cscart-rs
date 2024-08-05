use crate::prelude::*;
use crate::service::config::ServiceConfig;
use crate::types::Cart;
use crate::{impl_delete_by_id_method, impl_get_all_method, impl_get_by_id_method};

use serde::Deserialize;

pub type DeleteCartResponse = ();

pub struct CartService {
    config: ServiceConfig<Authenticated>,
}

impl CartService {
    pub fn with_config(service: ServiceConfig<Authenticated>) -> CartService {
        Self { config: service }
    }
}

#[derive(Deserialize, Debug)]
pub struct GetAllCartResponse {
    pub carts: Vec<Cart>,
}

impl_get_by_id_method!(CartService, Cart);
impl_get_all_method!(CartService, GetAllCartResponse);
impl_delete_by_id_method!(CartService, DeleteCartResponse);
