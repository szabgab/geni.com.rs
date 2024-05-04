use serde::Deserialize;

#[derive(PartialEq, Debug, Deserialize)]
pub struct Profile<'a> {
    pub id: &'a str,
    pub profile_url: &'a str,
    pub public: bool,
}
