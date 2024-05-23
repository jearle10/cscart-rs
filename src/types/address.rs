use serde::{Deserialize, Serialize};

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
