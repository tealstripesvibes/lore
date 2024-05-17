use crate::ui;

pub fn section_header(text: &str) -> String {
    return format!("======\n{}\n======", text)
}

pub fn page() {
    let license = 
        "All rights reserved.\n \
         This book \n \
         or any portion thereof \n \
         may not be reproduced \n \
         without \n \
         the express written permission of the author,\n \
         except for educational purposes \n \
         or the use of brief quotations in social media \n \
         to show everyone your good taste in literature";

    println!("{}", section_header(license));
    ui::pause();

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
    ui::pause();
}
