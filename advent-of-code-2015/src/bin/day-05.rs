fn main() {
    let input = std::fs::read_to_string("input/day-05.txt").unwrap();

    let string_vec: Vec<&str> = input.split_ascii_whitespace().collect();

    // PART 1
    let mut nice: u128 = 0;

    for string in &string_vec {
        if string.contains("ab")
            || string.contains("cd")
            || string.contains("pq")
            || string.contains("xy")
        {
            continue;
        }

        let mut appeared_twice: bool = false;
        let mut prev_ch = ' ';

        for ch in string.chars() {
            if prev_ch == ch {
                appeared_twice = true;
                break;
            }
            prev_ch = ch;
        }

        let mut vowels = 0;

        for ch in string.chars() {
            if ch == 'a' || ch == 'e' || ch == 'i' || ch == 'o' || ch == 'u' {
                vowels += 1
            }
        }

        if appeared_twice && vowels >= 3 {
            nice += 1;
        }
    }

    println!("{}", nice);

    // PART 2

    const CHARS: usize = 16;
    let mut nice: u128 = 0;

    for string in &string_vec {
        let mut con1 = false;
        let mut iter_l = string.chars();
        let mut iter_r = string.chars().skip(2);

        for _iter_r_idx in 2..CHARS {
            if iter_l.next().unwrap() == iter_r.next().unwrap() {
                con1 = true;
                break;
            }
        }

        let mut con2 = false;
        let mut iter_l = string.chars().skip(2);
        let mut iter_r = string.chars().skip(3);

        for iter_r_idx in 3..CHARS {
            let pair: String =
                iter_l.next().unwrap().to_string() + &iter_r.next().unwrap().to_string();

            if string[..iter_r_idx - 1].contains(&pair) {
                con2 = true;
                break;
            }
        }

        if con1 && con2 {
            nice += 1;
        }
    }

    println!("{}", nice);
}
