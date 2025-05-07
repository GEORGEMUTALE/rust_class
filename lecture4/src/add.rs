pub fn addition(a:i32, b:i32) -> i32{
    a + b
}

// #[cfg(test)]  //this is an attribute that separates the mod fns and the test
// mod tests {
//     use super::*;  // imports everything from the parent module

//     #[test]
//     fn test_add() {
//         assert_eq!(addition(2, 2), 4); //macro that compares equality
//     }

//     #[test]
//     // #[ignore = "We shall look at this test tomorrow"]
//     fn test_negative_add(){
//         assert_eq!(addition(-1,-3), -4);
//     }
// }