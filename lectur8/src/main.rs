struct Person {
    name: String,
    age:i32
}


impl Person {
    fn new(name: &str) -> Self {Person {
        name: name.to_string(),
        age: 20
    }
}
    fn display_name(&self){
        println!("Your name is {}", self.name)
    }

    fn display_age(&self){
        println!("My age is {}", self.age)
    }
}

fn main(){
    let person1 = Person::new("Martin");
    person1.display_name();

    let person2 = Person::new("George");
    person2.display_age();
    person2.display_name();
}