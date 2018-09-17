extern crate racer_testutils;
use racer_testutils::*;

#[test]
fn finds_def_of_i64() {
    let src = "
    fn main() {
        let a: i64~ = 0;
    }
    ";
    let got = get_definition(src, None);
    assert_eq!(got.matchstr, "i64", "{:?}", got);
}

#[test]
fn get_def_of_str_method() {
    let src = r#"
        fn check() {
            "hello".to_lowerca~se();
        }
    "#;

    let got = get_definition(src, None);
    assert_eq!("to_lowercase", got.matchstr);
}

#[test]
fn completes_liballoc_method_for_str() {
    let src = r#"
    fn in_let() {
        let foo = "hello";
        foo.to_lowerc~
    }
    "#;

    let got = get_only_completion(src, None);
    assert_eq!(got.matchstr, "to_lowercase");
}

#[test]
fn completes_libcore_method_for_str() {
    let src = r#"
    fn in_let() {
        let foo = "hello";
        foo.le~
    }
    "#;

    let got = get_only_completion(src, None);
    assert_eq!(got.matchstr, "len");
}
