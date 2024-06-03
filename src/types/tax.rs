use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Tax {
    tax: String,                           // Tax name
    tax_id: String,                        // Tax ID
    regnumber: Option<String>,             // Registration number
    priority: Option<String>,              // Tax priority
    price_includes_tax: Option<String>,    // If Y, the tax will be included in product price (Y/N)
    display_including_tax: Option<String>, // Include the tax in product price (Y/N)
    address_type: Option<String>, // If S, the tax rates will depend on shipping address (S/B)
    status: Option<String>,       // Tax status (A for Active, D for Disabled, H for Hidden)
    lang_code: Option<String>,    // Language code for the tax name
}
