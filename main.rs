#[derive(Debug)]
struct Book {
    author: String,
    name: String,
}
impl Book {
    //Associated function
    pub fn book_data(author: String, name: String) -> Book {
        Book {
            author: author,
            name: name,
        }
    }
    fn bookinformation(&self) -> String {
        let info = format!("AuthorName:{}\nBook name{}\n", self.author, self.name);
        info
    }
}

fn main() {
    let book_01 = Book::book_data("Harry Potter".to_string(), "J.K Rowling".to_string());
    
    println!("{}", book_01.bookinformation());
}
