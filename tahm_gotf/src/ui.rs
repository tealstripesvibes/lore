use std::io::{stdin, stdout, Read, Write};

pub fn cls() {
    // clear the terminal screen with a control char
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

pub fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
    cls();
}

