mod pages;
mod ui;

fn page_1() {
    pages::page_001::page();
}

fn page_2() {
    pages::page_002::page();
}

fn main() {
    page_1();
    page_2();
}
