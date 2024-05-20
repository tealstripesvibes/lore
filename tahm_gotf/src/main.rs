mod ui;
mod pages;

use std::io;

fn main() {
    let page_selection = read_page_number();
    let mut attention = read_attention_span();

    let mut i         = 0;
    if attention < 1 { return; }; i += 1; if i >= page_selection { pages::page_001::page(); attention -= 1; }
    if attention < 1 { return; }; i += 1; if i >= page_selection { pages::page_002::page(); attention -= 1; }
    if attention < 1 { return; }; i += 1; if i >= page_selection { pages::page_003::page(); attention -= 1; }
    if attention < 1 { return; }; i += 1; if i >= page_selection { pages::page_004::page(); attention -= 1; }
    if attention < 1 { return; }; i += 1; if i >= page_selection { pages::page_005::page(); attention -= 1; }
    if attention < 1 { return; }; i += 1; if i >= page_selection { pages::page_006::page(); attention -= 1; }
    if attention < 1 { return; }; i += 1; if i >= page_selection { pages::page_007::page(); attention -= 1; }
    if attention < 1 { return; }; i += 1; if i >= page_selection { pages::page_008::page(); attention -= 1; }
    if attention < 1 { return; }; i += 1; if i >= page_selection { pages::page_009::page(); attention -= 1; }
    if attention < 1 { return; }; i += 1; if i >= page_selection { pages::page_010::page(); attention -= 1; }
    if attention < 1 { return; }; i += 1; if i >= page_selection { pages::page_011::page(); attention -= 1; }
    if attention < 1 { return; }; i += 1; if i >= page_selection { pages::page_012::page(); attention -= 1; }
    if attention < 1 { return; }; i += 1; if i >= page_selection { pages::page_013::page(); attention -= 1; }

    if attention > 0 {
        println!("thanks for reading!");
    }
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