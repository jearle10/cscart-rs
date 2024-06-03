use crate::utils::serde_utils::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize)]
pub struct Payment {
    payment: String,
    a_surcharge: String,
    #[serde(deserialize_with = "deserialize_string_or_int_to_i32")]
    company_id: i32,
    description: Option<String>,
    image: Option<String>,
    instructions: Option<String>,
    lang_code: Option<String>,
    localisation: Option<String>,
    p_surcharge: Option<String>,
    payment_category: Option<String>,
    payment_id: Option<String>,
    postition: Option<String>,
    processor: Option<String>,
    processor_id: Option<String>,
    processor_params: Option<String>,
    processor_type: Option<String>,
    status: String,
    surcharge_title: String,
    #[serde(deserialize_with = "deserialize_string_or_int_to_vec_i32")]
    tax_ids: Vec<i32>,
    template: String,
    // usergroups_ids: Option<Vec<i32>>, // Todo
}
