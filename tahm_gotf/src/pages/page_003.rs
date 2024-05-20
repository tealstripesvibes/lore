use crate::ui;
use crate::pages;

#[derive(Debug)]
enum ChapterName {
    Prolegomenon,
    TheTrialByStarfire,
    InterregnumTheSecretHorrors,
    TheAlienGalaxy,
    InterregnumHemeraNyxInHerOwnTime,
    TheEffulgentRecrudescence,
}

impl std::fmt::Display for ChapterName {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ChapterName::Prolegomenon => write!(f, "{}", "Prolegomenon"),
            ChapterName::TheTrialByStarfire => write!(f, "{}", "The Trial By Starfire"),
            ChapterName::InterregnumTheSecretHorrors => write!(f, "{}", "Interregnum: The Secret Horrors!"),
            ChapterName::TheAlienGalaxy => write!(f, "{}", "The Alien Galaxy!"),
            ChapterName::InterregnumHemeraNyxInHerOwnTime => write!(f, "{}", "Interregnum: Hemera Nyx, In Her Own Time!"),
            ChapterName::TheEffulgentRecrudescence => write!(f, "{}", "The Effulgent Recrudescence"),
        }
    }
}

pub fn page() {
    let tribute = pages::page_002::section_header("For my Dad, and his Destination: Universe!");
    println!("{}", tribute);
    ui::pause(true);

    let chapters = [
        ChapterName::Prolegomenon,
        ChapterName::TheTrialByStarfire,
        ChapterName::InterregnumTheSecretHorrors,
        ChapterName::TheAlienGalaxy,
        ChapterName::InterregnumHemeraNyxInHerOwnTime,
        ChapterName::TheEffulgentRecrudescence,
    ].map(
        |name|
            format!("[{}]", name.to_string().to_uppercase())
    );

    let delimiter = "======";
    let delimiter = &format!("\n{}\n", delimiter)[..];
    println!("{}", pages::page_002::section_header(&chapters.join(delimiter)));
    ui::pause(true);
}
