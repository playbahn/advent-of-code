use regex::Regex;

#[derive(PartialEq)]
enum LastNest {
    Array,
    Object,
}

use LastNest::*;

fn main() {
    let mut sum: i128 = 0;
    let json: String = std::fs::read_to_string("input/day12.txt").unwrap();
    let re = Regex::new(r"-*\d+").unwrap();
    let numbers: Vec<&str> = re.find_iter(&json[..]).map(|number| number.as_str()).collect();
    
    for num in numbers {
        sum = sum + num.parse::<i128>().unwrap();
    }
    
    println!("{}", sum);
    
    let mut last_nest: Vec<LastNest> = Vec::new();
    let mut nested_sums: Vec<i128> = Vec::new();
    let mut cursor: std::str::Chars<'_> = json.chars();
    let mut ch: char;
    let mut skip_nested_obj: u8 = 0;
    let mut re: [char; 2] = ['\0'; 2];
    let mut cur_num: String = String::new();
    loop {
        ch = dbg!(cursor.next().unwrap());
        
        match ch {
            '{' => {
                last_nest.push(Object);
                nested_sums.push(0);
            },
            
            '[' => {
                last_nest.push(Array);
                nested_sums.push(0);
            },
            
            ']' => {
                last_nest.pop();
                *nested_sums.last_mut().unwrap() += nested_sums.pop().unwrap();
            },
            
            '}' => {
                last_nest.pop();
                if last_nest.len() == 0 {
                    break;
                }
                *nested_sums.last_mut().unwrap() += nested_sums.pop().unwrap();
            },
            
            'r' => {
                re[0] = 'r';
            },
            
            'e' if re[0] == 'r' => {
                re[1] = 'e';
            },

            'd' if (re == ['r', 'e'] && Some(&Object) == last_nest.last()) => {
                skip_nested_obj += 1;
                while skip_nested_obj > 0 {
                    ch = cursor.next().unwrap();
                    if ch == '{' {
                        skip_nested_obj += 1;
                    } else if ch == '}' {
                        skip_nested_obj -= 1;
                    }
                }
                last_nest.pop();
                nested_sums.pop();
                re = ['\0', '\0'];
            },
            
            '-' | '0'..='9' => {
                cur_num.push(ch);
            },
            
            _=> {
                re = ['\0', '\0'];
                if cur_num.len() > 0 {
                    *nested_sums.last_mut().unwrap() += cur_num.parse::<i128>().unwrap();
                    cur_num.clear();
                }
            },
        }

        println!("{:?}", nested_sums);
    }
    
}
