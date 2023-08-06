#![cfg(test)]

use crate::caf;

#[test]
fn test_const_anon_functions() {
    const RESULT: i32 = caf!(|a: i32, b: i32| -> i32 { a + b })(1, 2);

    assert_eq!(RESULT, 1 + 2);
}
