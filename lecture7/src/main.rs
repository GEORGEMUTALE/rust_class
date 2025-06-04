struct Book {
    title: String,
    author: String,
    pages: u32,
}

fn main() {
    // Create an instance of the Book struct
    let book = Book {
        title: String::from("The Rust Programming Language"),
        author: String::from("Steve Klabnik and Carol Nichols"),
        pages: 519,
    };

    println!("Book: {} by {} ({} pages)", 
             book.title, book.author, book.pages);
}