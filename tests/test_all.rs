#[cfg(test)]
mod tests {
    use geni::Profile;
    use std::fs::read_to_string;

    #[test]
    fn read_profile() {
        let raw = read_to_string("tests/profile-122248213.json").unwrap();

        let prof: Profile = serde_json::from_str(&raw).unwrap();

        assert_eq!(
            prof,
            Profile {
                id: "profile-122248213",
                profile_url: "https://www.geni.com/people/Super-Testy-Tester/6000000012102785219",
                public: false,
            }
        );
    }
}
