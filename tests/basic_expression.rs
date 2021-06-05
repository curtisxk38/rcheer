mod common;

#[test]
fn test_basic_expression() {
    let input = "(4 + 5 + -1) * 2";
    assert!(match common::run_test(input) {
        common::TestResult::Execution(status_code) => status_code == 16,
        _ => false,
    });
}

