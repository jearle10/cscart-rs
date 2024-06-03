use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LanguageDetails {
    lang_id: String,
    lang_code: String,
    name: String,
    country_code: String,
    direction: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Language {
    #[serde(rename = "en")]
    En(LanguageDetails),
    #[serde(rename = "rs")]
    Rs(LanguageDetails),
}
