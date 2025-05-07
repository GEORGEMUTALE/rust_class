use lecture4::*;
// use lecture4::{add,div}; //new keyword use

fn main() {
    let sum = add::addition(5, 3);  
    println!("sum is {}", sum);

    match div::div(5, 3) {     
        Ok(result) => println!("Division is {}", result),
        Err(e) => println!("Error: {}", e),
    }
}