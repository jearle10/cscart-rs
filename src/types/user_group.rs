use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct UserGroup {
    #[serde(flatten)]
    groups: HashMap<String, UserGroupDetails>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserGroupDetails {
    usergroup_id: Option<String>, // The ID of the user group
    #[serde(rename = "type")]
    group_type: UserType, // The type of the user group
    status: UserGroupStatus,      // The status of the user group
    usergroup: Option<String>,    // The name of the user group
    privileges: Option<Vec<String>>, // The privileges of the members of this group
}

#[derive(Serialize, Deserialize, Debug)]
enum UserType {
    A, // Admins
    C, // Customers
}

#[derive(Serialize, Deserialize, Debug)]
enum UserGroupStatus {
    A, // Active
    H, // Hidden
    D, // Disabled
}
