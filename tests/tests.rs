extern crate coq_imports_lib;
use coq_imports_lib::*;

#[test]
fn test_get_package_components() -> () {
    assert_eq!(vec!["Foo", "Bar", "Baz"],
        package_components(&mut String::from("Foo.Bar.Baz")[..]));
}

// https://stackoverflow.com/questions/38183551/concisely-initializing-a-vector-of-strings
// macro_rules! vec_of_string_ref {
//     ($($x:expr),*) => (vec![$(&$x.to_string()),*]);
// }

#[test]
fn test_matching_prefix() -> () {
    // assert_eq!(0, matching_prefix(vec_of_string_ref!["x"],
    //                               vec_of_string_ref!["z"]));
    // assert_eq!(1, matching_prefix(vec_of_string_ref!["x"],
    //                               vec_of_string_ref!["x"]));
}

#[test]
fn test_max_matching_line() -> () {
    // for (s, v) in vec![
    //     ("", vec![(0, String::from("Require Import Foo.Bar.Baz."))])
    // ].iter().cloned() {
    //     assert_eq!(s, v.iter().cloned())
    // }
    assert_eq!(0, max_matching_line("", vec![(0, String::from("Require Import Foo.Bar.Baz."))].iter().cloned()));
    assert_eq!(1, max_matching_line("Foo.Bar", vec![(0, String::from("Require Import Foo.Baa.Tre")), (1, String::from("Require Import Foo.Bar.Sky"))].iter().cloned()));
}
