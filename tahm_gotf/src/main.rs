mod ui;
mod pages;

use std::io;

fn main() {
    let page_selection = read_page_number();
    let mut attention = read_attention_span();

    ui::cls();

    for i in 1..=13 {
        if i < page_selection { continue; }
        if !can_continue(&attention) { return; };
        match i {
             1 => pages::page_001::page(),
             2 => pages::page_002::page(),
             3 => pages::page_003::page(),
             4 => pages::page_004::page(),
             5 => pages::page_005::page(),
             6 => pages::page_006::page(),
             7 => pages::page_007::page(),
             8 => pages::page_008::page(),
             9 => pages::page_009::page(),
            10 => pages::page_010::page(),
            11 => pages::page_011::page(),
            12 => pages::page_012::page(),
            13 => pages::page_013::page(),
            _ => ()
        }
        attention = use_attention(attention);
    }

    if attention > 0 {
        println!("thanks for reading!");
    }
}

fn use_attention(attention: u8) -> u8 {
    attention - 1
}

fn can_continue(attention: &u8) -> bool {
    *attention > 0
}

fn read_page_number() -> u8 {
    println!("Which page do you want to start on?");
    let mut page_number = String::new();
    io::stdin().read_line(&mut page_number).expect("Failed to read input");
    let page_number: u32 = page_number.trim().parse().expect("Could not parse page number");
    page_number as u8
}

fn read_attention_span() -> u8 {
    println!("How long is your attention span?");
    let mut attention_span = String::new();
    io::stdin().read_line(&mut attention_span).expect("Failed to read input");
    let page_number: u32 = attention_span.trim().parse().expect("Could not parse page number");
    page_number as u8
}