use std::fs;

fn main() {
    let ins: String = fs::read_to_string("input/day-02.txt").unwrap();
    let mut btn: char = '5';
    let mut code: String = String::new();
    for ins in ins.lines() {
        for ins in ins.chars() {
            match (ins, btn) {
                // PART 1
                // ('U', '4'..='9') => btn = (btn as u8 - 3) as char,
                // ('D', '1'..='6') => btn = (btn as u8 + 3) as char,
                // ('L', '2' | '3' | '5' | '6' | '8' | '9') => btn = (btn as u8 - 1) as char,
                // ('R', '1' | '2' | '4' | '5' | '7' | '8') => btn = (btn as u8 + 1) as char,
                // PART 2
                ('U', '6'..='8') => btn = (btn as u8 - 4) as char,
                ('U', 'A'..='C') => btn = (btn as u8 - 11) as char,
                ('U', '3' | 'D') => btn = (btn as u8 - 2) as char,
                ('D', '2'..='4') => btn = (btn as u8 + 4) as char,
                ('D', '6'..='8') => btn = (btn as u8 + 11) as char,
                ('D', '1' | 'B') => btn = (btn as u8 + 2) as char,
                ('L', '3' | '4' | '6'..='9' | 'B' | 'C') => btn = (btn as u8 - 1) as char,
                ('R', '2' | '3' | '5'..='8' | 'A' | 'B') => btn = (btn as u8 + 1) as char,
                _ => (),
            }
        }
        code.push(btn);
    }

    println!("{code}")
}
