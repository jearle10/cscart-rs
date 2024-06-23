use crate::prelude::*;
use crate::service::config::ServiceConfig;
use crate::types::Order;
use crate::{
    impl_create_method, impl_delete_by_id_method, impl_get_all_method, impl_get_by_id_method,
    impl_update_by_id_method,
};
use serde::Deserialize;
use serde_json::Value;

pub struct OrderService {
    config: ServiceConfig<Authenticated>,
}

impl OrderService {
    pub fn with_config(service: ServiceConfig<Authenticated>) -> OrderService {
        Self { config: service }
    }
}

#[derive(Deserialize, Debug)]
pub struct GetAllOrderResponse {
    pub orders: Vec<Order>,
}

impl_create_method!(OrderService, Order);
impl_get_by_id_method!(OrderService, Order);
impl_get_all_method!(OrderService, GetAllOrderResponse);
impl_update_by_id_method!(OrderService, Value);
impl_delete_by_id_method!(OrderService, Value);
