mod ui;
mod pages;

use std::io;

fn main() {
    println!("Please enter starter page");

    let mut input = String::new();
    
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let page_number: u32 = input.trim().parse().expect("Could not parse page number");

    let mut i = 0;
    i += 1; if i >= page_number { pages::page_001::page(); }
    i += 1; if i >= page_number { pages::page_002::page(); }
    i += 1; if i >= page_number { pages::page_003::page(); }
    i += 1; if i >= page_number { pages::page_004::page(); }
    i += 1; if i >= page_number { pages::page_005::page(); }
    i += 1; if i >= page_number { pages::page_006::page(); }
    i += 1; if i >= page_number { pages::page_007::page(); }
    i += 1; if i >= page_number { pages::page_008::page(); }
    i += 1; if i >= page_number { pages::page_009::page(); }
    i += 1; if i >= page_number { pages::page_010::page(); }
    i += 1; if i >= page_number { pages::page_011::page(); }
    i += 1; if i >= page_number { pages::page_012::page(); }
    i += 1; if i >= page_number { pages::page_013::page(); }
}
