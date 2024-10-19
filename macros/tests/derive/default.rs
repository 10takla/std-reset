#![allow(unused)]

use std::default;

use std_reset_macros::Default;

#[test]
fn named_structure() {
    #[derive(Debug, Default, PartialEq)]
    struct Named {
        #[default(String::from("Ferris"))]
        first: String,
        #[default("Ferris")]
        second: &'static str,
        #[default(8_9999_999_999)]
        third: u128,
        fourth: Option<String>,
        #[default(Some(32))]
        fifth: Option<u32>,
    }
    assert_eq!(
        Named::default(),
        Named {
            first: "Ferris".to_string(),
            second: "Ferris",
            third: 8_9999_999_999,
            fourth: None,
            fifth: Some(32),
        }
    );
}

#[test]
fn unnamed_structure() {
    #[derive(Debug, Default, PartialEq)]
    struct Unnamed(
        #[default(String::from("Ferris"))] String,
        #[default("Ferris")] &'static str,
        #[default(8_9999_999_999)] u128,
        Option<String>,
        #[default(Some(32))] Option<u32>,
    );
    assert_eq!(
        Unnamed::default(),
        Unnamed(
            "Ferris".to_string(),
            "Ferris",
            8_9999_999_999,
            None,
            Some(32),
        )
    );
}

#[test]
fn unit_structure() {
    #[derive(Debug, Default, PartialEq)]
    struct Unit;
    assert_eq!(Unit::default(), Unit);
}

#[test]
fn with_generics() {
    #[derive(Debug, Default, PartialEq)]
    struct Data<A, B, C, D>(A, B, C, D);
    assert_eq!(Data::<i32, i64, i128, usize>::default(), Data(0, 0, 0, 0));
}

#[test]
fn enum_() {
    #[derive(Default, PartialEq, Debug)]
    enum Units {
        #[default]
        One,
        Two,
    }
    assert_eq!(Units::default(), Units::One);

    #[derive(Default, PartialEq, Debug)]
    enum Unnamed {
        #[default]
        One(#[default(10)] i32),
        Two,
    }
    assert_eq!(Unnamed::default(), Unnamed::One(10));

    #[derive(PartialEq, Debug)]
    struct UnnamedStruct;

    #[derive(Default, PartialEq, Debug)]
    enum Named {
        One,
        #[default]
        Two {
            #[default(UnnamedStruct)]
            first: UnnamedStruct,
        },
    }
    assert_eq!(
        Named::default(),
        Named::Two {
            first: UnnamedStruct
        }
    );
}
