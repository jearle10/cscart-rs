use crate::prelude::*;
use crate::service::config::ServiceConfig;
use crate::types::Block;
use crate::{
    impl_create_method, impl_delete_by_id_method, impl_get_all_method, impl_get_by_id_method,
    impl_update_by_id_method,
};
use serde_json::Value;

pub struct BlockService {
    config: ServiceConfig<Authenticated>,
}

impl BlockService {
    pub fn with_config(service: ServiceConfig<Authenticated>) -> BlockService {
        Self { config: service }
    }
}

pub type GetAllBlockResponse = Vec<Block>;

impl_create_method!(BlockService, Block);
impl_get_by_id_method!(BlockService, Block);
impl_get_all_method!(BlockService, GetAllBlockResponse);
impl_update_by_id_method!(BlockService, Value);
impl_delete_by_id_method!(BlockService, Value);
