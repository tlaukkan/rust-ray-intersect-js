use intersect;

#[test]
fn test_add() {
    assert_eq!(intersect::add(1.0, 2.0), 3.0);
}

#[test]
fn test_bad_add() {
    // This assert would fire and test will fail.
    // Please note, that private functions can be tested too!
    assert_eq!(intersect::bad_add(1.0, 2.0), 3.0);
}
