use crate::types::address::{BillingAddress, ShippingAddress};
use crate::types::product::Product;
use crate::types::user::User;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Cart {
    user_id: String,
    firstname: Option<String>,
    lastname: Option<String>,
    date: String,
    ip_address: String,
    cart_products: Vec<String>,
    total: String,
    order_id: Option<String>,
    user_data: Vec<UserData>,
    products: Vec<Product>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UserData {
    #[serde(flatten)]
    user_details: User,
    #[serde(flatten)]
    billing_address: BillingAddress,
    #[serde(flatten)]
    shipping_address: ShippingAddress,
    referrer: Option<String>,
    phone: String,
    fax: String,
    url: String,
    tax_exempt: String,
    lang_code: String,
    birthday: String,
    purchase_timestamp_from: Option<String>,
    purchase_timestamp_to: Option<String>,
    reponsible_email: Option<String>,
    janrain_identifier: Option<String>,
    usergroups: Vec<String>,
    profile_id: Option<String>,
    profile_type: Option<String>,
    points: String,
}
