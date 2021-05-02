mod common;

#[test]
fn test_basic_expression() {
    let input = "(4 + 5 + -1) * 2";
    assert!(common::run_test(input) == 16);
}