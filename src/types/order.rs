use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Order {
    pub order_id: Option<String>,
    pub issuer_id: Option<String>,
    pub user_id: Option<String>,
    pub is_parent_order: Option<String>,
    pub parent_order_id: Option<String>,
    pub company_id: Option<String>,
    pub timestamp: Option<String>,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub status: Option<String>,
    pub total: Option<String>,
    pub invoice_id: Option<String>,
    pub credit_memo_id: Option<String>,
    pub points: Option<String>,
    #[serde(flatten)]
    pub shipping_address: ShippingAddress,
    #[serde(flatten)]
    pub billing_address: BillingAddress,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct OrderDetails {
    #[serde(flatten)]
    pub order: Order,
    #[serde(flatten)]
    pub shipping_address: ShippingAddress,
    #[serde(flatten)]
    pub billing_address: BillingAddress,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShippingAddress {
    pub s_address: Option<String>,
    pub s_adddress_2: Option<String>,
    pub s_city: Option<String>,
    pub s_country: Option<String>,
    pub s_country_descr: Option<String>,
    pub s_firstname: Option<String>,
    pub s_lastname: Option<String>,
    pub s_phone: Option<String>,
    pub s_state: Option<String>,
    pub s_state_descr: Option<String>,
    pub s_zipcode: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BillingAddress {
    pub b_address: Option<String>,
    pub b_adddress_2: Option<String>,
    pub b_city: Option<String>,
    pub b_country: Option<String>,
    pub b_country_descr: Option<String>,
    pub b_firstname: Option<String>,
    pub b_lastname: Option<String>,
    pub b_phone: Option<String>,
    pub b_state: Option<String>,
    pub b_state_descr: Option<String>,
    pub b_zipcode: Option<String>,
}
