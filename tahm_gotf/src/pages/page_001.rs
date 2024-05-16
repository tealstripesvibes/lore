use crate::ui;

pub fn page() {
    let series_title = "The Adventures of Hemera Nyx";
    let preposition  = "in";
    let title        = "The Galaxy of The Future";
    let book_title   = format!("{} {} {}", series_title, preposition, title);
    
    let author_logo  = format!("= [{}] =", "RSK");
    
    println!("{}\n{}", book_title, author_logo);

    ui::pause();
}
