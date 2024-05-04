use serde::Deserialize;
use std::collections::HashMap;

#[derive(PartialEq, Debug, Deserialize)]
pub struct Name {
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub display_name: String,
}

#[derive(PartialEq, Debug, Deserialize)]
pub struct Profile {
    pub id: String,
    pub profile_url: String,
    pub public: bool,
    pub guid: String,
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
    pub display_name: String,
    pub name: String,
    pub account_type: String,
    pub deleted: bool,
    pub is_curator: bool,
    pub names: HashMap<String, Name>,
}

#[derive(PartialEq, Debug, Deserialize)]
pub struct Document {
    pub id: String,
    pub guid: String,
    pub created_at: String,
    pub updated_at: String,
    pub content_type: String,
    pub public: bool,
    pub url: String,
    pub title: String,
    pub description: String,
}
