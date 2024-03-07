use std::fs;

fn main() {
    let list: String = fs::read_to_string("input.txt").unwrap();
    let code_ch: usize = list.split_ascii_whitespace().collect::<String>().len();
    let mut mem_ch: isize = 0;
    let mut escape_encountered: bool = false;

    for line in list.lines().map(|line| {
        line.trim_matches('"')
    }) {
        for ch in line.chars() {
            mem_ch += 1;
            
            if escape_encountered {
                match ch {
                    '\\' | '\"' => mem_ch -= 1,
                    'x' => mem_ch -= 3,
                    _ => {},
                }

                escape_encountered = false;
            } else if ch == '\\' {
                escape_encountered = true;
            }
        }
    }

    println!("{}", code_ch as isize - mem_ch);
    
    let mut encoded_ch: usize = code_ch;
    
    for line in list.lines() {
        encoded_ch += 2;
        
        for ch in line.chars() {
            if ch == '"' || ch == '\\' {
                encoded_ch += 1;
            }
        }
    }

    println!("{}", encoded_ch - code_ch);
}
