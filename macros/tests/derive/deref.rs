use std_reset_macros::Deref;

#[test]
fn deref_every_type_structure() {
    #[derive(Debug, Deref)]
    struct Wrapper1(pub i32);
    assert_eq!(*Wrapper1(1), 1);

    #[derive(Debug, Deref)]
    struct Wrapper2(pub i32, #[deref] pub &'static str);
    assert_eq!(*Wrapper2(1, "1"), "1");

    #[derive(Debug, Deref)]
    struct Wrapper3 {
        pub first: i32,
        #[deref]
        pub second: &'static str,
    }
    assert_eq!(
        *Wrapper3 {
            first: 1,
            second: "1"
        },
        "1"
    );
}
