#[cfg(test)]
mod tests {
    use geni::Document;
    use geni::Name;
    use geni::Profile;
    use std::collections::HashMap;
    use std::fs::read_to_string;

    fn read_profile_from_file(path: &str) -> Profile {
        let raw = read_to_string(path).unwrap();
        serde_json::from_str::<Profile>(&raw).unwrap()
    }

    fn read_document_from_file(path: &str) -> Document {
        let raw = read_to_string(path).unwrap();
        serde_json::from_str::<Document>(&raw).unwrap()
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
                guid: String::from("6000000012102785219"),
                first_name: String::from("Super"),
                middle_name: String::from("Sev"),
                last_name: String::from("Tester"),
                display_name: String::from("Super Testy Tester"),
                name: String::from("Super Testy Tester"),
                account_type: String::from("basic"),
                deleted: false,
                is_curator: false,
                names: HashMap::from([
                    (
                        String::from("en-US"),
                        Name {
                            first_name: String::from("Super"),
                            last_name: String::from("Tester"),
                            middle_name: Some(String::from("Sev")),
                            display_name: String::from("Super Testy Tester"),
                        }
                    ),
                    (
                        String::from("de"),
                        Name {
                            first_name: String::from("Uber"),
                            last_name: String::from("Testlich"),
                            display_name: String::from("Uber Testlich"),
                            middle_name: None,
                        }
                    ),
                    (
                        String::from("id"),
                        Name {
                            first_name: String::from("Indonesian"),
                            last_name: String::from("Tester"),
                            display_name: String::from("Indonesian Tester"),
                            middle_name: None,
                        }
                    )
                ]),
            }
        );
    }

    #[test]
    fn read_document() {
        let document = read_document_from_file("tests/document-339953.json");
        assert_eq!(
            document,
            Document {
                id: String::from("document-339953"),
                guid: String::from("6000000011320334348"),
                created_at: String::from("1296320586"),
                updated_at: String::from("1417732225"),
                content_type: String::from("image/png"),
                public: true,
                url: String::from("https://www.geni.com/api/document-339953"),
                title: String::from("Jaworow AGAD Births 1877-96"),
                description: String::from(
                    "Jewish Records Indexing - Poland\nhttp://www.jewishgen.org/jri-pl/index.htm"
                ),
            }
        );
    }
}
