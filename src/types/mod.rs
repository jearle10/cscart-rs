use serde::de::Deserializer;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
pub struct Product {
    product: String,
    category_ids: Vec<i32>,
    main_category: i32,
    price: String,
    #[serde(deserialize_with = "deserialize_string_or_int")]
    pub company_id: i32,
    pub status: String,
    #[serde(deserialize_with = "deserialize_string_or_int")]
    amount: i32,
    avail_since: String,
    box_height: Option<i32>,
    #[serde(deserialize_with = "deserialize_string_or_int")]
    length: i32,
    box_width: Option<i32>,
    details_layout: String,
    edp_shipping: String,
    exceptions_type: String,
    free_shipping: String,
    full_description: Option<String>,
    //  TODO image_pairs
    is_dep: Option<String>,
    lang_code: Option<String>,
    #[serde(deserialize_with = "deserialize_string_or_float")]
    list_price: f32,
    list_qty_count: String,
    localisation: Option<String>,
    low_avail_limit: String,
    // TODO main_pair
    max_qty: String,
    max_items_in_box: Option<i32>,
    meta_description: Option<String>,
    meta_keywords: Option<String>,
    min_items_in_box: Option<i32>,
    #[serde(deserialize_with = "deserialize_string_or_int")]
    min_qty: i32,
    options_type: Option<String>,
    out_of_stock_actions: String,
    page_title: Option<String>,
    point_price: Option<String>,
    popularity: Option<String>,
    product_code: String,
    // TODO product_features
    #[serde(deserialize_with = "deserialize_string_or_int")]
    pub product_id: i32,
    pub promo_text: Option<String>,
    #[serde(deserialize_with = "deserialize_string_or_int")]
    qty_step: i32,
    return_period: String,
    sales_amount: Option<String>,
    search_words: Option<String>,
    seo_name: String,
    shared_product: Option<String>,
    shipping_freight: String,
    shipping_params: String,
    short_description: Option<String>,
    // #[serde(deserialize_with = "deserialize_string_or_int_to_vec_i32")]
    // tax_ids: Vec<i32>,
    timestamp: String,
    tracking: Option<String>,
    unlimited_download: String,
    updated_timestamp: String,
    #[serde(deserialize_with = "deserialize_string_or_int_to_vec_i32")]
    usergroup_ids: Vec<i32>,
    #[serde(deserialize_with = "deserialize_string_or_float")]
    pub weight: f32,
    pub zero_price_action: String,
}

fn deserialize_string_or_int<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Value::deserialize(deserializer)?;
    let parse_result = match value {
        Value::String(string) => string.parse::<i64>().ok(),
        Value::Number(number) => number.as_i64(),
        _ => None,
    };

    match parse_result {
        Some(number) => Ok(number as i32),
        None => Err(serde::de::Error::custom("Could not parse")),
    }
}

fn deserialize_string_or_int_to_vec_i32<'de, D>(deserializer: D) -> Result<Vec<i32>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Value::deserialize(deserializer)?;

    // Convert each variant to an array before parsing
    let value_array = match value.clone() {
        Value::Array(x) => x,
        Value::String(y) => vec![Value::String(y)],
        Value::Number(z) => vec![Value::Number(z)],
        _ => vec![],
    };

    let mut parse_results: Vec<Option<i64>> = vec![];

    for item in value_array {
        match item {
            Value::Array(values) => {
                for i in values.iter() {
                    match i {
                        Value::String(a) => parse_results.push(a.parse::<i64>().ok()),
                        Value::Number(b) => parse_results.push(b.as_i64()),
                        _ => {}
                    }
                }
            }
            Value::String(string) => parse_results.push(string.parse::<i64>().ok()),
            Value::Number(number) => parse_results.push(number.as_i64()),
            _ => {}
        }
    }

    match parse_results {
        bad_result if bad_result.contains(&None) => Err(serde::de::Error::custom(format!(
            "Could not parse, {}",
            value
        ))),
        mut good_result => Ok(good_result.iter_mut().map(|x| x.unwrap() as i32).collect()),
    }
}

fn deserialize_string_or_float<'de, D>(deserializer: D) -> Result<f32, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Value::deserialize(deserializer)?;
    let parse_result = match value {
        Value::String(string) => string.parse::<f64>().ok(),
        Value::Number(number) => number.as_f64(),
        _ => None,
    };

    match parse_result {
        Some(number) => Ok(number as f32),
        None => Err(serde::de::Error::custom("Could not parse data")),
    }
}
