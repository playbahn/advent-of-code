#[derive(Clone, Copy, PartialEq)]
enum State {
    On,
    Off,
}

struct Coord {
    x: u32,
    y: u32,
}

struct Instruction<'a> {
    config: &'a str,
    start: Coord,
    end: Coord,
}

use State::*;

fn main() {
    // let mut grid: [[State; 1000]; 1000] = [[OFF; 1000]; 1000];
    let input = std::fs::read_to_string("input/day-06.txt").unwrap();

    let ins_vec: Vec<&str> = input.lines().collect();

    let mut ins_vec_main: Vec<Instruction> = Vec::new();

    for ins in ins_vec {
        let mut ins_iter = ins.split_ascii_whitespace();

        let config: &str = ins_iter.next().unwrap();
        let mut start = ins_iter.next().unwrap().split_terminator(',');
        let start: Coord = Coord {
            x: start.next().unwrap().parse().unwrap(),
            y: start.next().unwrap().parse().unwrap(),
        };

        let mut end = ins_iter.next().unwrap().split_terminator(',');
        let end: Coord = Coord {
            x: end.next().unwrap().parse().unwrap(),
            y: end.next().unwrap().parse().unwrap(),
        };

        ins_vec_main.push(Instruction { config, start, end });
    }

    let mut grid: Vec<[State; 1000]> = vec![[Off; 1000]; 1000];

    for instruction in &ins_vec_main {
        match instruction.config {
            "ON" => {
                for x in instruction.start.x..instruction.end.x + 1 {
                    for y in instruction.start.y..instruction.end.y + 1 {
                        grid[x as usize][y as usize] = On;
                    }
                }
            }

            "OFF" => {
                for x in instruction.start.x..instruction.end.x + 1 {
                    for y in instruction.start.y..instruction.end.y + 1 {
                        grid[x as usize][y as usize] = Off;
                    }
                }
            }
            "TOGGLE" => {
                for x in instruction.start.x..instruction.end.x + 1 {
                    for y in instruction.start.y..instruction.end.y + 1 {
                        if grid[x as usize][y as usize] == On {
                            grid[x as usize][y as usize] = Off;
                        } else {
                            grid[x as usize][y as usize] = On;
                        }
                    }
                }
            }
            _ => {}
        }
    }

    let mut lit: u32 = 0;

    for x in grid {
        for y in x {
            if y == On {
                lit += 1;
            }
        }
    }

    println!("{}", lit);

    let mut grid: Vec<[u32; 1000]> = vec![[0; 1000]; 1000];

    for instruction in &ins_vec_main {
        match instruction.config {
            "ON" => {
                for x in instruction.start.x..instruction.end.x + 1 {
                    for y in instruction.start.y..instruction.end.y + 1 {
                        grid[x as usize][y as usize] += 1;
                    }
                }
            }

            "OFF" => {
                for x in instruction.start.x..instruction.end.x + 1 {
                    for y in instruction.start.y..instruction.end.y + 1 {
                        if grid[x as usize][y as usize] > 0 {
                            grid[x as usize][y as usize] -= 1;
                        }
                    }
                }
            }
            "TOGGLE" => {
                for x in instruction.start.x..instruction.end.x + 1 {
                    for y in instruction.start.y..instruction.end.y + 1 {
                        grid[x as usize][y as usize] += 2;
                    }
                }
            }
            _ => {}
        }
    }

    let mut brightness: u32 = 0;

    for x in grid {
        for y in x {
            brightness += y;
        }
    }

    println!("{}", brightness);
}
