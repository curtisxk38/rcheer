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

#[test]
fn test_less_than() {
    let input = "5 < 6";
    assert!(match common::run_test(input) {
        common::TestResult::Execution(status_code) => status_code == 1,
        _ => false,
    });
}


#[test]
fn test_equal_fail() {
    let input = "5 == 7";
    assert!(match common::run_test(input) {
        common::TestResult::Execution(status_code) => status_code == 0,
        _ => false,
    });
}

#[test]
fn test_if() {
    let input = "if 2 > 1 { 1} else { 0 }";
    assert!(match common::run_test(input) {
        common::TestResult::Execution(status_code) => status_code == 1,
        _ => false,
    });
}

#[test]
fn test_if_type_error() {
    let input = "if 2 > 1 { 1 } else { 0 > 1 }";
    assert!(match common::run_test(input) {
        common::TestResult::TypeError => true,
        _ => false,
    });
}