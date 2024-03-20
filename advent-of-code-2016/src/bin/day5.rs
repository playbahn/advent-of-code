fn main() {
    const ID: &str = "cxdnnyjw";
    // let mut pswd: String = String::new();
    let mut pswd: [(char, bool); 8] = [('\0', false); 8];
    let mut pswd_entries: u8 = 0;

    for index in 0.. {
        if pswd_entries == 8 {
            break;
        }
        let digest: md5::Digest = md5::compute(ID.to_owned() + &index.to_string());
        let digest: String = format!("{:?}", digest);
    
        if &digest[..5] == "00000" {
            if let Ok(idx) = digest[5..6].parse::<usize>() {
                if idx < 8 && !pswd[idx].1 {
                    pswd[idx] = (digest[6..7].parse().unwrap(), true);
                    pswd_entries += 1;
                }
            }
        }
    }

    println!("{:?}", pswd);
}
