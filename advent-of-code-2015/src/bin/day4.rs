fn main() {
    let input = String::from("yzbqklnj");

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
