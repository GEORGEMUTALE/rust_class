use lecture4::add::addition;
// this is an attribute below
#[test]
fn test_add_positive_numbers() {
    assert_eq!(addition(2, 3), 5);
}

#[test]
fn test_add_negative_numbers() {
    assert_eq!(addition(-1, -1), -2);
}

#[test]
fn test_add_mixed_numbers() {
    assert_eq!(addition(5, -3), 2);
}

#[test]
fn test_add_zero() {
    assert_eq!(addition(0, 0), 0);
    assert_eq!(addition(5, 0), 5);
}