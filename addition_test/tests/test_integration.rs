use addition_test;

mod common;

#[test]
fn cela_ajoute_deux() {
    common::param();
    assert_eq!(4, addition_test::ajouter_deux(2));
}