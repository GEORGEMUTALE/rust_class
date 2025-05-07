use chrono::prelude::*;
use std::io;
fn main(){

    let current_year:i32 = Local::now().year();

    println!("Enter your birth year");
    let mut birth_year = String::new();
    io::stdin()
        .read_line(&mut birth_year)
        .expect("Failed to read input");

        	// converting the input into number
            //trim removes whitespaces and parse returns Result<T,E>

    let  birth_year:i32 = birth_year.trim().parse().expect("Please enter a valid year");
    
    let age = current_year - birth_year;
    println!("You are {} years old.", age);
}