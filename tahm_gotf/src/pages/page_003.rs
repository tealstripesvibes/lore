use crate::ui;
use crate::pages;


pub fn page() {
    let tribute = pages::page_002::section_header("For my Dad, and his Destination: Universe!");
    println!("{}", tribute);
    ui::pause(true);

    let chapters = [
        "PROLEGOMENON",
        "THE TRIAL BY STARFIRE",
        "INTERREGNUM: THE SECRET HORRORS!",
        "THE ALIEN GALAXY!",
        "INTERREGNUM: HEMERA NYX, IN HER OWN TIME!",
        "THE EFFULGENT RECRUDESCENCE",
    ];

    let delimiter = "======";
    let delimiter = &format!("\n{}\n", delimiter)[..];
    println!("{}", pages::page_002::section_header(&chapters.map(|s| format!("[{}]", s)).join(delimiter)));
    ui::pause(true);
}
