use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Setting {
    name: String,
    description: Option<String>,
    object_id: Option<String>,
    section_id: Option<String>,
    section_tab_id: Option<String>,
    value: Option<String>,
    edition_type: Option<String>,
    handler: Option<String>,
    is_global: Option<String>,
    object_type: Option<String>,
    position: Option<String>,
    section_name: Option<String>,
    section_tab_name: Option<String>,
    tooltip: Option<String>,
    #[serde(rename = "type")]
    setting_type: String,
    variants: Option<Vec<Variant>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Variant {
    value: String,
    label: Option<String>,
}
