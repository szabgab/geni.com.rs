

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;
    use geni::Profile;

    #[test]
    fn read_profile() {
        let raw = read_to_string("tests/profile-122248213.json").unwrap();

        let prof: Profile = serde_json::from_str(&raw).unwrap();

        assert_eq!(prof, Profile {id: String::from("profile-122248213")});
    }
}