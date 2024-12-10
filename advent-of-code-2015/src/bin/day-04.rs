fn main() {
    let input = std::fs::read_to_string("input/day-04.txt").unwrap();

    for num in 1.. {
        let secret_key = input.clone() + &num.to_string();
        let digest = md5::compute(secret_key);
        let digest = format!("{:?}", digest);
        if digest.starts_with("000000") {
            println!("{}", num);
            break;
        }
    }
}
