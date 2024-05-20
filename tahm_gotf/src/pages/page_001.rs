use crate::ui;

#[derive(Debug)]
enum Word {
    The,
    Adventures,
    Of,
    Hemera,
    Nyx,
    In,
    Galaxy,
    Future,
}

impl std::fmt::Display for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Word::Of => write!(f, "{}", "of"),
            Word::In => write!(f, "{}", "in"),
            Word::Future => write!(f, "{}", "Future!"),
            _ => write!(f, "{:?}", self),
        }
    }
}


pub fn page() {
    let series_title = get_series_title();
    let collection_title = get_collection_title();
    let book_title = format!("{} {} {}", series_title, Word::In, collection_title);

    let author_logo = format!("= [{}] =", "RSK");

    println!("{}\n{}", book_title, author_logo);

    ui::pause(true);
}

fn get_series_title() -> String {
    let series_title = [
        Word::The,
        Word::Adventures,
        Word::Of,
        Word::Hemera,
        Word::Nyx,
    ];
    let series_title = series_title.map(|w| w.to_string()).join(" ");
    series_title
}

fn get_collection_title() -> String {
    let title = [
        Word::The,
        Word::Galaxy,
        Word::Of,
        Word::The,
        Word::Future,
    ];
    let title = title.map(|w| w.to_string()).join(" ");
    title
}
