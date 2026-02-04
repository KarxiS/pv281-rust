#[test]
fn test_basic() {
    assert_eq!(2 + 2, 4);
}

#[test]
fn test_vec() {
    let v = [1, 2, 3];
    assert_eq!(v.len(), 3);
}
