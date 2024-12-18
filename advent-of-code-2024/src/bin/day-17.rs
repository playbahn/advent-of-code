fn main() {
    let mut a: u128 = std::fs::read_to_string("input/day-17.txt")
        .unwrap()
        .lines()
        .next()
        .unwrap()[12..]
        .parse()
        .unwrap();
    
    let (mut b, mut c);

    let mut out: Vec<u128> = vec![];

    loop {
        b = (a % 8) ^ 2;
        c = a >> b;
        b = b ^ c ^ 3;
        out.push(b % 8);
        a >>= 3;
        if a == 0 {
            break;
        }
    }

    println!("{out:?}");
}
