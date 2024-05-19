use crate::ui;
use crate::pages;


pub fn page() {
    let tribute = pages::page_002::section_header("For my Dad, and his Destination: Universe!");
    println!("{}", tribute);
    ui::pause();

    let chapters = [
        "PROLEGOMENON",
        "THE TRIAL BY STARFIRE",
        "INTERREGNUM: THE SECRET HORRORS!",
        "THE ALIEN GALAXY!",
        "INTERREGNUM: HEMERA NYX, IN HER OWN TIME!",
        "THE EFFULGENT RECRUDESCENCE",
    ];

    println!("{}", pages::page_002::section_header(&chapters.map(|s| format!("[{}]", s)).join("======\n")));
    ui::pause();
}
