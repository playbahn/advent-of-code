fn main() {
    let mut input: String = String::from("1113122113");
    
    for _ in 0..50 {
        // println!("curr_input = {}", input);
        
        let input_len: usize = input.len();
        
        let mut next_input: String = String::new();
        
        let mut occurences: u8 = 1;
        let mut last_digit: char = input.chars().next().unwrap();
        
        for (idx, curr_digit) in input.chars().skip(1).enumerate() {
            if curr_digit == last_digit {
                occurences += 1;
            } else {
                next_input.extend(parse_u8_to_string(occurences).chars());
                next_input.push(last_digit);
                
                occurences = 1;
                last_digit = curr_digit;
            }

            if idx + 1 == input_len - 1 {
                next_input.extend(parse_u8_to_string(occurences).chars());
                next_input.push(last_digit);
            }
        }
        
        // println!("next_input = {}", next_input);
        
        input = next_input;
    }
    
    println!("{}", input.len());

    // println!("{:?}", parse_u8_to_string(124));
}

fn parse_u8_to_string(mut arg: u8) -> String {
    let mut res: String = String::new();

    while arg != 0 {
        res.push(char::from(arg % 10 + 48));
        arg /= 10;
    }

    res.chars().rev().collect()
}
