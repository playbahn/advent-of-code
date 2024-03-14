fn main() {
    // (3010, 3019)
    let (mut row, mut col): (u16, u16) = (6, 6);

    let mut prev: u128 = 27995004;
    let mut cur: u128;

    loop {
        row -= 1;
        col += 1;

        if row == 0 {
            row = col;
            col = 1;
        }

        cur = (prev * 252533) % 33554393;
        prev = cur;

        if row == 3010 && col == 3019 {
            break;
        }

        if row + col > 6029 {
            panic!("row: {row}\ncol: {col}")
        }
    }

    println!("{cur}");
}
