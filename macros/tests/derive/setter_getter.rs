use std_reset_macros::{Default, Getter, Setter};

#[test]
fn setter_and_getter_default() {
    #[derive(Setter, Getter, Clone, Copy, Default, PartialEq, Debug)]
    struct Tmp {
        first: i32,
        second: i32,
    }

    let mut tmp = Tmp::default().set_first(2).set_second(2);
    tmp.set_second(5);

    assert_eq!(
        tmp,
        Tmp {
            first: 2,
            second: 5
        }
    );

    assert_eq!(tmp.get_first(), 2);
    assert_eq!(tmp.get_second(), 5);
}

#[test]
fn setter_and_getter_with_exclude_fields() {
    #[derive(Setter, Getter, Clone, Copy, Default, PartialEq, Debug)]
    struct Tmp {
        #[exclude_getter]
        first: i32,
        #[exclude_setter]
        #[exclude_getter]
        second: i32,
        #[exclude_setter]
        third: i32,
    }

    let tmp = Tmp::default().set_first(2);

    assert_eq!(
        tmp,
        Tmp {
            first: 2,
            second: i32::default(),
            third: i32::default()
        }
    );

    assert_eq!(tmp.get_third(), 0);
}

#[test]
fn setter_and_getter_with_include_field() {
    #[derive(Setter, Getter, Clone, Default, Copy, PartialEq, Debug)]
    struct Tmp {
        first: i32,
        #[include_setter]
        #[include_getter]
        second: i32,
        #[include_setter]
        third: i32,
    }

    let tmp = Tmp::default().set_second(2).set_third(2);

    assert_eq!(
        tmp,
        Tmp {
            first: i32::default(),
            second: 2,
            third: 2
        }
    );

    assert_eq!(tmp.get_second(), 2);
}
