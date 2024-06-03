use crate::utils::serde_utils::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CallRequest {
    pub request_id: Option<String>,
    pub email: Option<String>,
    pub phone: String,
    pub user_id: Option<String>,
    pub order_id: Option<String>,
    pub product_id: Option<String>,
    pub timestamp: Option<String>,
    pub status: Option<String>,
    pub name: Option<String>,
    pub time_from: Option<String>,
    pub time_to: Option<String>,
    pub notes: Option<String>,
    // pub cart_products: Option<Vec<CartProduct>>, // Todo
    pub order_status: Option<String>,
    pub product: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CartProduct {
    pub product_id: String,
    pub amount: String,
    pub price: String,
    pub product: String,
}
