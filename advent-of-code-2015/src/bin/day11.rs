use std::collections::HashSet;

fn main() {
    // let mut pswd: Vec<char> = Vec::from_iter("vzbxkghb".chars());
    // let mut pswd: Vec<char> = Vec::from_iter("vzbxxyzz".chars());
    let mut pswd: Vec<char> = Vec::from_iter("vzbxxzaa".chars());
    let mut no_iol: bool;
    let mut increasing_straight: bool;
    let mut non_overlapping_pairs: bool;
    let mut pair_one: [char; 2];
    let pairs: HashSet<&str> = HashSet::from(["aa", "bb", "cc", "dd", "ee", "ff", "gg", "hh", "ii", "jj", "kk", "ll", "mm", "nn", "oo", "pp", "qq", "rr", "ss", "tt", "uu", "vv", "ww", "xx", "yy", "zz"]);
    loop {
        no_iol = false;
        increasing_straight = false;
        non_overlapping_pairs = false;
        pair_one = ['\0'; 2];

        if !(pswd.contains(&'i') || pswd.contains(&'o') || pswd.contains(&'l')) {
            no_iol = true;
        } else {
            for (idx, ch) in pswd.iter_mut().enumerate() {
                if ['i', 'o', 'l'].contains(ch) {
                    *ch = unsafe { char::from_u32_unchecked(*ch as u32 + 1) };
                    pswd[idx + 1 ..].fill('a');
                    // had_iol = true;
                    break;
                }
            }
        }

        for ch_window in pswd.windows(3) {
            if ch_window[0] as u32 + 1 == ch_window[1] as u32
            && ch_window[1] as u32 + 1 == ch_window[2] as u32 {
                increasing_straight = true;
                break;
            }
        }

        for ch_window in pswd.windows(2) {
            if pairs.contains(&String::from_iter(ch_window)[..]) {
                (pair_one[0], pair_one[1]) = (ch_window[0], ch_window[1]);
                break;
            }
        }

        for ch_window in pswd.windows(2) {
            if pairs.contains(&String::from_iter(ch_window)[..])
            && (pair_one[0], pair_one[1]) != (ch_window[0], ch_window[1]) {
                non_overlapping_pairs = true;
                break;
            }
        }
        
        if no_iol && (increasing_straight && non_overlapping_pairs) {
            break;
        }

        mutate(&mut pswd);
    }

    println!("{}", pswd.iter().collect::<String>());
}

fn mutate(pswd: &mut Vec<char>) {
    for ch in pswd.iter_mut().rev() {
        if *ch == 'z' {
            *ch = 'a';
        } else {
            *ch = unsafe { char::from_u32_unchecked(*ch as u32 + 1) };
            break;
        }
    }
}
