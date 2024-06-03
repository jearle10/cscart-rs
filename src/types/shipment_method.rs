use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct ShipmentMethod {
    shipping: String,
    company_id: Option<String>,
    delivery_time: Option<String>,
    icon: Option<MainPair>,
    localization: Option<String>,
    max_weight: Option<String>,
    min_weight: Option<String>,
    position: Option<String>,
    rate_calculation: Option<String>,
    rates: Option<Rates>,
    service_id: Option<String>,
    service_params: Option<Value>,
    shipping_id: Option<String>,
    status: Option<String>,
    tax_ids: Option<String>,
    usergroup_ids: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct MainPair {
    pair_id: Option<String>,
    image_id: Option<String>,
    detailed_id: Option<String>,
    position: Option<String>,
    object_id: Option<String>,
    object_type: Option<String>,
    is_new: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Rates {
    rate_value: RateValue,
}

#[derive(Serialize, Deserialize, Debug)]
struct RateValue {
    #[serde(rename = "C")]
    c: Value,
    #[serde(rename = "I")]
    i: Value,
    #[serde(rename = "W")]
    w: Value,
}
