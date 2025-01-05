use std::collections::HashMap;

enum Signal {
    Raw(u16),
    Ops(Op),
}

enum Op {
    Wire(String),
    And(String, String),
    Or(String, String),
    Not(String),
    LShift(String, u8),
    RShift(String, u8),
}

use Op::*;
use Signal::*;

fn main() {
    let input = std::fs::read_to_string("input/day-07.txt").unwrap();
    let input: Vec<&str> = input.lines().collect();

    let mut map: HashMap<String, Signal> = HashMap::new();

    for ins in input {
        let wire: &str = ins.split_ascii_whitespace().last().unwrap();

        let ins_len: usize = ins.split_ascii_whitespace().count();

        if ins_len == 3 {
            // SIGNAL
            match ins.split_ascii_whitespace().next().unwrap() {
                raw_sig if raw_sig.parse::<u16>().is_ok() => {
                    map.insert(wire.to_owned(), Raw(raw_sig.parse().unwrap()));
                }

                wire_sig => {
                    map.insert(wire.to_owned(), Ops(Wire(wire_sig.to_string())));
                }
            }
        } else if ins_len == 4 {
            // NOT
            let wire_sig: &str = ins.split_ascii_whitespace().nth(1).unwrap();
            map.insert(wire.to_string(), Ops(Not(wire_sig.to_string())));
        } else {
            // [`ins_len`] == 5     // SHIFT, AND, OR
            let vec5: Vec<&str> = ins.split_ascii_whitespace().collect();

            if ins.contains("OR") {
                let in1: &str = vec5[0];
                let in2: &str = vec5[2];

                map.insert(wire.to_owned(), Ops(Or(in1.to_string(), in2.to_string())));
            } else if ins.contains("AND") {
                let in1: &str = vec5[0];
                let in2: &str = vec5[2];

                map.insert(wire.to_owned(), Ops(And(in1.to_owned(), in2.to_owned())));
            } else {
                // SHIFT
                match vec5[1] {
                    "LSHIFT" => {
                        map.insert(
                            wire.to_owned(),
                            Ops(LShift(vec5[0].to_owned(), vec5[2].parse().unwrap())),
                        );
                    }
                    "RSHIFT" => {
                        map.insert(
                            wire.to_owned(),
                            Ops(RShift(vec5[0].to_owned(), vec5[2].parse().unwrap())),
                        );
                    }
                    _ => {}
                }
            }
        }
    }

    // println!("{:?}", map);
    // let a = get_signal("a".to_string(), &mut map);

    // println!("part 1: {}", a);

    // map.insert("b".to_string(), Raw(a));
    println!("part 2: {}", get_signal("a".to_string(), &mut map));
}

fn get_signal(wire: String, map: &mut HashMap<String, Signal>) -> u16 {
    let val: &mut Signal = map.get_mut(&wire).unwrap();

    match val {
        Raw(signal) => *signal,
        Ops(op) => match op {
            Wire(input) => {
                let raw: u16 = get_signal(input.to_string(), map);

                map.insert(wire, Raw(raw));

                raw
            }

            And(in1, in2) => {
                let in1: String = in1.clone();
                let in2: String = in2.clone();

                let raw1: u16 = if let Ok(one) = in1.parse::<u16>() {
                    one
                } else {
                    get_signal(in1, map)
                };
                let raw2: u16 = get_signal(in2, map);

                map.insert(wire, Raw(raw1 & raw2));

                raw1 & raw2
            }

            Or(in1, in2) => {
                let in1: String = in1.clone();
                let in2: String = in2.clone();
                let raw1: u16 = get_signal(in1, map);
                let raw2: u16 = get_signal(in2, map);

                map.insert(wire, Raw(raw1 | raw2));

                raw1 | raw2
            }

            Not(input) => {
                let raw: u16 = !get_signal(input.to_string(), map);

                map.insert(wire, Raw(raw));

                raw
            }

            LShift(input, bits) => {
                let bits: u8 = *bits;
                let raw: u16 = get_signal(input.to_string(), map) << bits;

                map.insert(wire, Raw(raw));

                raw
            }

            RShift(input, bits) => {
                let bits: u8 = *bits;
                let raw: u16 = get_signal(input.to_string(), map) >> bits;

                map.insert(wire, Raw(raw));

                raw
            }
        },
    }
}
