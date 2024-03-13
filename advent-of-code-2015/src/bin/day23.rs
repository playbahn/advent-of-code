use std::fs;

fn main() {
    let mut ra: u128;
    ra = 0;
    // PART 2
    // ra = 1;
    let mut rb: u128 = 0;
    let program: String = fs::read_to_string("input/day23.txt").unwrap();
    let program: Vec<String> = program
        .lines()
        .map(|line| line.to_owned())
        .collect::<Vec<String>>();

    let mut cur_ins: isize = 0;

    while let Some(ins) = program.get(cur_ins as usize) {
        let ins: Vec<&str> = ins.split_ascii_whitespace().collect();
        match ins[0] {
            "hlf" => {
                match ins[1] {
                    "a" => ra >>= 1,
                    _ => panic!(),
                }
                cur_ins += 1;
            }
            "tpl" => {
                match ins[1] {
                    "a" => ra += ra << 1,
                    _ => panic!(),
                }
                cur_ins += 1;
            }
            "inc" => {
                match ins[1] {
                    "a" => ra += 1,
                    "b" => rb += 1,
                    _ => panic!(),
                }
                cur_ins += 1;
            }
            "jmp" => cur_ins += ins[1].parse::<isize>().unwrap(),
            "jie" => match ins[1] {
                "a," if ra & 1 == 0 => cur_ins += ins[2].parse::<isize>().unwrap(),
                _ => cur_ins += 1,
            },
            "jio" => match ins[1] {
                "a," if ra == 1 => cur_ins += ins[2].parse::<isize>().unwrap(),
                _ => cur_ins += 1,
            },
            _ => panic!(),
        }
    }

    println!("{}", rb);
}
