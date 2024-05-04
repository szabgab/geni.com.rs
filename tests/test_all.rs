#[cfg(test)]
mod tests {
    use geni::Profile;
    use std::fs::read_to_string;

    fn read_profile_from_file(path: &str) -> Profile {
        let raw = read_to_string(path).unwrap();
        serde_json::from_str::<Profile>(&raw).unwrap()
    }

    #[test]
    fn read_profile() {
        let prof = read_profile_from_file("tests/profile-122248213.json");
        assert_eq!(
            prof,
            Profile {
                id: String::from("profile-122248213"),
                profile_url: String::from(
                    "https://www.geni.com/people/Super-Testy-Tester/6000000012102785219"
                ),
                public: false,
            }
        );
    }
}
