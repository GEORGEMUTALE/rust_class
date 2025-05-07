// div.rs
//result is an enum type
//enum is a group different variants of different data type

pub fn div(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Division by zero".to_string()) //.to_string()is a method
    } else {
        Ok(a / b)
    }
}