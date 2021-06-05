mod common;

#[test]
fn test_basic_expression() {
    let input = "(4 + 5 + -1) * 2";
    assert!(match common::run_test(input) {
        common::TestResult::Execution(status_code) => status_code == 16,
        _ => false,
    });
}

#[test]
fn type_error() {
    let input = "5 < (5 < 5)";
    assert!(match common::run_test(input) {
        common::TestResult::TypeError => true,
        _ => false,
    });
}

#[test]
fn type_error2() {
    let input = "5 == (5 < 5)";
    assert!(match common::run_test(input) {
        common::TestResult::TypeError => true,
        _ => false,
    });
}