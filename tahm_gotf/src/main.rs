fn page_1() -> String {
    let series_title = "The Adventures of Hemera Nyx";
    let preposition  = "in";
    let title        = "The Galaxy of The Future";
    let book_title   = format!("{} {} {}", series_title, preposition, title);
    
    let author_logo  = format!("= [{}] =", "RSK");
    
    return format!("{}\n{}", book_title, author_logo);
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

    return format!("{}\n\n{}", license, trigger_warnings);
}

fn main() {
    println!("\n\n{}\n\n", page_1());
    println!("\n\n{}\n\n", page_2());
}
