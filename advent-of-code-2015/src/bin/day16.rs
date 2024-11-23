use std::fs;

fn main() {
    const CHILDREN: i8 = 3;
    const CATS: i8 = 7;
    const SAMOYEDS: i8 = 2;
    const POMERANIANS: i8 = 3;
    const AKITAS: i8 = 0;
    const VIZSLAS: i8 = 0;
    const GOLDFISH: i8 = 5;
    const TREES: i8 = 3;
    const CARS: i8 = 2;
    const PERFUMES: i8 = 1;

    let aunt_list: String = fs::read_to_string("input/day16.txt").unwrap();

    let mut aunts: Vec<i8> = Vec::new();
    
    let mut probability: i8;

    for aunt in aunt_list.lines() {
        let aunt: Vec<&str> = aunt
            .split(&[' ', ',', ':'][..])
            .filter(|substring| substring != &"")
            .collect();

        probability = 0;
        
        for features in aunt.chunks(2).skip(1) {
            match features[0] {
                "children" => {
                    if features[1].parse::<i8>().unwrap() == CHILDREN {
                        probability += 1;
                    } else {
                        probability -= 1;
                    }
                },
                "cats" => {
                    if features[1].parse::<i8>().unwrap() > CATS {
                        probability += 1;
                    } else {
                        probability -= 1;
                    }
                },
                "samoyeds" => {
                    if features[1].parse::<i8>().unwrap() == SAMOYEDS {
                        probability += 1;
                    } else {
                        probability -= 1;
                    }
                },
                "pomeranians" => {
                    if features[1].parse::<i8>().unwrap() < POMERANIANS {
                        probability += 1;
                    } else {
                        probability -= 1;
                    }
                },
                "akitas" => {
                    if features[1].parse::<i8>().unwrap() == AKITAS {
                        probability += 1;
                    } else {
                        probability -= 1;
                    }
                },
                "vizslas" => {
                    if features[1].parse::<i8>().unwrap() == VIZSLAS {
                        probability += 1;
                    } else {
                        probability -= 1;
                    }
                },
                "goldfish" => {
                    if features[1].parse::<i8>().unwrap() < GOLDFISH {
                        probability += 1;
                    } else {
                        probability -= 1;
                    }
                },
                "trees" => {
                    if features[1].parse::<i8>().unwrap() > TREES {
                        probability += 1;
                    } else {
                        probability -= 1;
                    }
                },
                "cars" => {
                    if features[1].parse::<i8>().unwrap() == CARS {
                        probability += 1;
                    } else {
                        probability -= 1;
                    }
                },
                "perfumes:" => {
                    if features[1].parse::<i8>().unwrap() == PERFUMES {
                        probability += 1;
                    } else {
                        probability -= 1;
                    }
                },
                _ => {},
            }
        }
        
        aunts.push(probability);
    }

    // println!("{:?}", aunts);
    
    let mut highest_probability: i8 = i8::MIN;
    let mut probable_ids: Vec<usize> = Vec::new();

    for cur_prob in &aunts {
        highest_probability = highest_probability.max(*cur_prob);
    }

    for (cur_id, cur_prob) in aunts.iter().enumerate() {
        if *cur_prob == highest_probability {
            probable_ids.push(cur_id);
        }
    }

    println!("{:?}", probable_ids);
}
