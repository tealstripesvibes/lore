use crate::ui;

pub fn section_header(text: &str) -> String {
    return format!("======\n{}\n======", text);
}

#[derive(Debug)]
enum PotentialTrigger {
    AbusiveRelationship(),
    Alcohol(),
    Assault(),
    AttemptedMurder(),
    Blood(),
    Bullying(),
    Death(),
    Depression(),
    Drugs(),
    EmotionalAbuse(),
    Eugenics(),
    Fire(),
    Gore(),
    GunViolence(),
    Hallucinations(),
    Murder(),
    PhysicalAbuse(),
    Profanity(),
    PTSD(),
    Skeletons(),
    Spiders(),
    Violence(),
}

impl std::fmt::Display for PotentialTrigger {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            PotentialTrigger::AbusiveRelationship() => write!(f, "{}", "Abusive Relationship"),
            PotentialTrigger::AttemptedMurder() => write!(f, "{}", "Attempted Murder"),
            PotentialTrigger::EmotionalAbuse() => write!(f, "{}", "Emotional Abuse"),
            PotentialTrigger::GunViolence() => write!(f, "{}", "Gun Violence"),
            PotentialTrigger::PhysicalAbuse() => write!(f, "{}", "Physical Abuse"),
            _ => write!(f, "{:?}", self),
        }
    }
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
    ui::pause(true);

    let trigger_warnings = [
        PotentialTrigger::AbusiveRelationship(),
        PotentialTrigger::Alcohol(),
        PotentialTrigger::Assault(),
        PotentialTrigger::AttemptedMurder(),
        PotentialTrigger::Blood(),
        PotentialTrigger::Bullying(),
        PotentialTrigger::Death(),
        PotentialTrigger::Depression(),
        PotentialTrigger::Drugs(),
        PotentialTrigger::EmotionalAbuse(),
        PotentialTrigger::Eugenics(),
        PotentialTrigger::Fire(),
        PotentialTrigger::Gore(),
        PotentialTrigger::GunViolence(),
        PotentialTrigger::Hallucinations(),
        PotentialTrigger::Murder(),
        PotentialTrigger::PhysicalAbuse(),
        PotentialTrigger::Profanity(),
        PotentialTrigger::PTSD(),
        PotentialTrigger::Skeletons(),
        PotentialTrigger::Spiders(),
        PotentialTrigger::Violence(),
    ];

    let bug_report_memo = "(Please contact RSK at RSK.Author@gmail.com if there are any missing warnings)";
    let trigger_warnings = format!(
        "Trigger Warnings:\n{}\n{}",
        trigger_warnings
            .map(|potential_trigger: PotentialTrigger| potential_trigger.to_string())
            .join("\n"),
        bug_report_memo
    );

    println!("{}\n", trigger_warnings);
    ui::pause(true);
}
