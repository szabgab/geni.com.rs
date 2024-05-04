use serde::Deserialize;

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
}
