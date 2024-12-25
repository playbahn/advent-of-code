const INPUT: &str = "input/day-25.txt";

fn main() {
    let mut locks: Vec<[u8; 5]> = Vec::with_capacity(250);
    let mut keys: Vec<[u8; 5]> = Vec::with_capacity(250);

    std::fs::read_to_string(INPUT)
        .unwrap()
        .split("\n\n")
        .for_each(|schematic| match &schematic[..1] {
            "#" => {
                let mut pins = [0u8; 5];
                schematic
                    .lines()
                    .enumerate()
                    .skip(1)
                    .take(5)
                    .for_each(|line| {
                        line.1.char_indices().for_each(|pin| {
                            if pin.1 == '#' {
                                pins[pin.0] = line.0 as u8
                            }
                        })
                    });
                locks.push(pins);
            }
            "." => {
                let mut spaces = [0u8; 5];
                schematic
                    .lines()
                    .enumerate()
                    .skip(1)
                    .take(5)
                    .for_each(|line| {
                        line.1.char_indices().for_each(|space| {
                            if space.1 == '.' {
                                spaces[space.0] = line.0 as u8
                            }
                        })
                    });
                keys.push(spaces);
            }
            _ => panic!(),
        });

    let mut unique = 0u16;

    for lock in &locks {
        'key: for key in &keys {
            for (space, pin) in key.iter().zip(lock) {
                if space < pin {
                    continue 'key;
                }
            }
            unique += 1;
        }
    }

    println!("part1: {unique}")
}
