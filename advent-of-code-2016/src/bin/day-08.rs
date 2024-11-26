fn main() {
    let ins_series: Vec<String> = std::fs::read_to_string("input/day-08.txt")
        .unwrap()
        .lines()
        .map(|ip| ip.to_owned())
        .collect();

    let mut screen: [[bool; 50]; 6] = [[false; 50]; 6];
    let mut temp_col: [bool; 6] = [false; 6];
    let mut lit: u16 = 0;

    for ins in ins_series {
        let ins: Vec<&str> = ins.split_whitespace().collect();

        if ins[0] == "rect" {
            let (x_lim, y_lim): (usize, usize) = (
                ins[1][..ins[1].find('x').unwrap()].parse().unwrap(),
                ins[1][1 + ins[1].find('x').unwrap()..].parse().unwrap(),
            );

            (0..y_lim).for_each(|y| (0..x_lim).for_each(|x| screen[y][x] = true));
        } else {
            let axis_offset: usize = ins[2][2..].parse().unwrap();
            let rotate_count: usize = ins[4].parse().unwrap();

            match ins[1] {
                "row" => screen[axis_offset].rotate_right(rotate_count),
                "column" => {
                    (0..6).for_each(|y| temp_col[y] = screen[y][axis_offset]);
                    (0..6).for_each(|y| screen[(y + rotate_count) % 6][axis_offset] = temp_col[y]);
                }
                _ => panic!(),
            }
        }
    }

    (0..6).for_each(|y| {
        (0..50).for_each(|x| {
            if screen[y][x] {
                lit += 1;
                print!("■ ");
            } else {
                print!("  ");
            }
        });
        println!();
    });

    println!("lit: {lit}")
    
    
}
