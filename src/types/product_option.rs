use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct OptionVariant {
    variant_id: Option<String>,
    option_id: Option<String>,
    position: Option<String>,
    modifier: f64,
    modifier_type: Option<String>, // "A" for absolute, "P" for percentage
    weight_modifier: f64,
    weight_modifier_type: Option<String>, // "A" for absolute, "P" for percentage
    point_modifier: f64,
    point_modifier_type: Option<String>, // "A" for absolute, "P" for percentage
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProductOption {
    option_id: Option<String>,
    product_id: Option<String>,
    company_id: Option<String>,
    option_type: Option<String>, // "S", "R", "C", "I", "T", "F"
    inventory: Option<String>,   // "Y" for yes, "N" for no
    regexp: Option<String>,
    required: String,            // "Y" for yes, "N" for no
    multiupload: Option<String>, // "Y" for yes, "N" for no
    allowed_extensions: Option<String>,
    max_file_size: Option<String>,
    missing_variants_handling: Option<String>, // "M" for message, "H" for hide
    status: String,                            // "A" for active, "D" for disabled
    position: String,
    value: Option<String>,
    option_name: String,
    description: Option<String>,
    inner_hint: Option<String>,
    incorrect_message: Option<String>,
    comment: Option<String>,
    variants: Option<Vec<OptionVariant>>,
}
