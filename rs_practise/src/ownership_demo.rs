struct Book {
    pages: i32,
    rating: i32,
}

fn display_page_count(book: &Book) {
    println!("pages = {:?}", book.pages);
}

fn display_rating(book: &Book) {
    println!("rating = {:?}", book.rating);
}

pub fn ownership_demo() {
    let book = Book {
        pages: 100,
        rating: 5,
    };
    display_page_count(&book);
    display_rating(&book);
}
