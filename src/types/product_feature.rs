use crate::utils::serde_utils::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize)]
pub struct ProductFeature {
    #[serde(deserialize_with = "deserialize_string_or_int_to_i32")]
    company_id: i32,
    description: Option<String>,
    feature_type: Option<String>,
    purpose: Option<String>,
    feature_style: Option<String>,
    filter_style: Option<String>,
    categories_path: Option<String>,
    comparison: Option<String>,
    display_on_catalog: Option<String>,
    display_on_product: Option<String>,
    display_on_header: Option<String>,
    feature_id: Option<String>,
    full_description: Option<String>,
    group_position: Option<String>,
    parent_id: Option<String>,
    position: Option<String>,
    prefix: Option<String>,
    status: Option<String>,
    suffix: Option<String>,
    value: Option<String>,
    value_int: Option<String>,
    variant_id: Option<String>,
    variants: Option<String>,
}
