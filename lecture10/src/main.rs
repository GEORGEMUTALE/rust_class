// use std::fmt; //this helps to implement how to display your struct
#[derive(Debug)]
 //this is to default display
#[allow(dead_code)]
struct PhoneNumber {
    country_code:String,
    number:String
}

// struct Student {
//     id: String,
//     number: i32,
//     name: String,
//     age: i8,
//     status: bool,
// }

// implementation below is to design the way how things are displayed on the terminal
// impl fmt::Display for PhoneNumber {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "+{}{}", self.country_code, self.number)
//     }
// }

fn main() {
    let phone_number1 = PhoneNumber {
        country_code:49.to_string(), number:123445644.to_string()};

    // println!("My contact is {}", phone_number1);  //this is for the user
    println!("My contact is {:?}", phone_number1);
    // println!("My contact is {:#?}", phone_number1) //this developers
}