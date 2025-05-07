pub fn addition(a:i32, b:i32) -> i32{
    a + b
}
#[cfg(test)]  //this is an attribute that separates the mod fns and the test
mod tests {
    use super::*;  // imports everything from the parent module

    #[test]
    fn test_add() {
        assert_eq!(addition(2, 2), 4);
    }
}