use std_reset_macros::Default;

#[test]
fn for_named_structure() {
    #[derive(Debug, Default, PartialEq)]
    struct User {
        #[default_field("String::from(\"Ferris\")")]
        name: String,
        #[default_field("String::from(\"123FerF\")")]
        password: String,
        #[default_field("8_9999_999_999")]
        number: u128,
        email: Option<String>,
        #[default_field("Some(32)")]
        age: Option<u32>,
    }
    assert_eq!(
        User::default(),
        User {
            name: "Ferris".to_string(),
            password: "123FerF".to_string(),
            number: 8_9999_999_999,
            email: None,
            age: Some(32),
        }
    );
}

#[test]
fn for_tuple_structure() {
    #[derive(Debug, Default, PartialEq)]
    struct User(
        #[default_field("String::from(\"Ferris\")")] String,
        #[default_field("String::from(\"123FerF\")")] String,
        #[default_field("8_9999_999_999")] u128,
        Option<String>,
        #[default_field("Some(32)")] Option<u32>,
    );
    assert_eq!(
        User::default(),
        User(
            "Ferris".to_string(),
            "123FerF".to_string(),
            8_9999_999_999,
            None,
            Some(32),
        )
    );
}
