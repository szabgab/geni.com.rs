use serde::Deserialize;

#[derive(PartialEq, Debug, Deserialize)]
pub struct Profile {
    pub id: String,
    pub profile_url: String,
    pub public: bool,
}
