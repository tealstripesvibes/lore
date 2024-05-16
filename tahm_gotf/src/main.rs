use std::io::{stdin, stdout, Read, Write};

fn cls() {
    // clear the terminal screen with a control char
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
    cls();
}

fn page_1() {
    let series_title = "The Adventures of Hemera Nyx";
    let preposition  = "in";
    let title        = "The Galaxy of The Future";
    let book_title   = format!("{} {} {}", series_title, preposition, title);
    
    let author_logo  = format!("= [{}] =", "RSK");
    
    println!("{}\n{}", book_title, author_logo);
}

fn page_2() -> String {
    let license = 
        "All rights reserved.\n \
         This book \n \
         or any portion thereof \n \
         may not be reproduced \n \
         without the express written \n \
         permission of the author,\n \
         except for educational purposes \n \
         or the use of brief quotations in social media \n \
         to show everyone your good taste in literature";
    let license = format!("======\n{}\n======", license);

    println!("{}", license);
    pause();

    let trigger_warnings = [
        "Abusive Relationship", 
        "Alcohol", 
        "Assault",
        "Attempted Murder",
        "Blood",
        "Bullying",
        "Death",
        "Depression",
        "Drugs",
        "Emotional Abuse",
        "Eugenics",
        "Fire",
        "Gore",
        "Gun Violence",
        "Hallucinations",
        "Murder",
        "Physical Abuse",
        "Profanity",
        "PTSD",
        "Skeletons",
        "Spiders",
        "Violence",
    ];
    let bug_report_memo  = "(Please contact RSK at RSK.Author@gmail.com if there are any missing warnings)";
    let trigger_warnings = format!(
        "Trigger Warnings:\n{}\n{}", 
        trigger_warnings.join("\n"), 
        bug_report_memo
    );

    println!("{}\n", trigger_warnings);
    pause();

    return "".to_string();
}

fn main() {
    page_1();
    pause();
    page_2();
    cls();
}
