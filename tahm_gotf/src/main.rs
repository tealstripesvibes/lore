fn page_1() -> String {
    let series_title = "The Adventures of Hemera Nyx";
    let preposition  = "in";
    let title        = "The Galaxy of The Future";
    let book_title   = format!("{} {} {}", series_title, preposition, title);
    
    let author_logo  = format!("= [{}] =", "RSK");
    
    return format!("{}\n{}", book_title, author_logo);
}

fn main() {
    println!("{}", page_1());
}
