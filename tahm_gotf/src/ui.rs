use std::io::{stdin, stdout, Read, Write};

pub fn cls() {
    print!(
        "{esc}[2J{esc}[1;1H", // https://stackoverflow.com/a/34837038
        esc = 27 as char
    );
}

pub fn pause(do_actually: bool) {
    if !do_actually { return }
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
    cls();
}

