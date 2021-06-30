use crate::module_b;

pub fn add(x: i32, y: i32) -> i32 {
    x + y
}


#[test]
fn test_add() {
    assert_eq!(12 + 12, add(12, 12));
    assert_eq!(10 + 11, add(10, 11));
    assert_eq!(3 + 4, add(3, 4));
}