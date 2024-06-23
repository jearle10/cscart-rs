use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Auth {
    pub key: String,
    pub link: String,
}
