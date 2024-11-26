use std::{collections::HashSet, fs};

fn main() {
    let ins: String = fs::read_to_string("input/day-01.txt").unwrap();

    let (x1, y1, _): (i16, i16, _) = ins
        .split(&[',', ' '][..])
        .filter(|subslice| !subslice.is_empty())
        .fold((0, 0, "N"), |(x, y, facing), ins| {
            let steps: &i16 = &ins[1..].parse().unwrap();
            match (facing, &ins[..1]) {
                ("N", "L") | ("S", "R") => (x - steps, y, "W"),
                ("N", "R") | ("S", "L") => (x + steps, y, "E"),
                ("E", "L") | ("W", "R") => (x, y + steps, "N"),
                ("E", "R") | ("W", "L") => (x, y - steps, "S"),
                _ => panic!(),
            }
        });

    println!("{}", i16::abs(x1) + i16::abs(y1));

    let (mut x2, mut y2): (i16, i16) = (0, 0);
    let mut facing: char = 'N';
    let mut visited: HashSet<(i16, i16)> = HashSet::from([(0, 0)]);

    'main: for ins in ins
        .split(&[',', ' '][..])
        .filter(|subslice| !subslice.is_empty())
    {
        let steps: &i16 = &ins[1..].parse().unwrap();
        match (facing, &ins[..1]) {
            ('N', "L") | ('S', "R") => {
                for x in (x2 - steps..x2).rev() {
                    if !visited.insert((x, y2)) {
                        println!("{}", i16::abs(x) + i16::abs(y2));
                        break 'main;
                    }
                }
                x2 -= steps;
                facing = 'W';
            }
            ('N', "R") | ('S', "L") => {
                for x in (x2 + 1)..(x2 + steps + 1) {
                    if !visited.insert((x, y2)) {
                        println!("{}", i16::abs(x) + i16::abs(y2));
                        break 'main;
                    }
                }
                x2 += steps;
                facing = 'E';
            }
            ('E', "L") | ('W', "R") => {
                for y in (y2 + 1)..(y2 + steps + 1) {
                    if !visited.insert((x2, y)) {
                        println!("{}", i16::abs(x2) + i16::abs(y));
                        break 'main;
                    }
                }
                y2 += steps;
                facing = 'N';
            }
            ('E', "R") | ('W', "L") => {
                for y in (y2 - steps..y2).rev() {
                    if !visited.insert((x2, y)) {
                        println!("{}", i16::abs(x2) + i16::abs(y));
                        break 'main;
                    }
                }
                y2 -= steps;
                facing = 'S';
            }
            _ => panic!(),
        }
    }
}
