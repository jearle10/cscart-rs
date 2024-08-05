use crate::types::address::{BillingAddress, ShippingAddress};
use crate::types::user::User;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug)]
pub struct Cart {
    pub user_id: String,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub date: String,
    pub ip_address: String,
    pub cart_products: String,
    pub total: String,
    pub order_id: Option<String>,
    pub user_data: UserData,
    pub products: Vec<CartProduct>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CartProduct {
    pub item_id: String,
    pub item_type: String,
    pub product_id: String,
    pub amount: String,
    pub price: String,
    pub product: String,
    pub extra: HashMap<String, Value>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UserData {
    #[serde(flatten)]
    pub user_details: User,
    #[serde(flatten)]
    pub billing_address: BillingAddress,
    #[serde(flatten)]
    pub shipping_address: ShippingAddress,
    pub referrer: Option<String>,
    pub phone: Option<String>,
    pub fax: String,
    pub url: String,
    pub tax_exempt: String,
    pub lang_code: String,
    pub birthday: String,
    pub purchase_timestamp_from: Option<String>,
    pub purchase_timestamp_to: Option<String>,
    pub reponsible_email: Option<String>,
    pub janrain_identifier: Option<String>,
    pub usergroups: Vec<String>,
    pub profile_id: Option<String>,
    pub profile_type: Option<String>,
    pub points: i32,
}
