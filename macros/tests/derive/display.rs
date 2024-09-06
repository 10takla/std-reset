use std_reset_macros::Display;

#[test]
fn display() {
    #[derive(Display, Debug)]
    struct Exmpl {}

    assert_eq!(format!("{:?}", Exmpl {}), format!("{}", Exmpl {}));
}
