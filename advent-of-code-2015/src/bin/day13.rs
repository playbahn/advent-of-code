use std::{collections::{HashMap, HashSet}, fs, i32::MIN};

fn main() {
    let all_stats: String = fs::read_to_string("input/day13.txt").unwrap();
    let mut stats: HashMap<&str, HashMap<&str, i32>> = HashMap::new();
    let mut people: HashSet<&str> = HashSet::new();
    let mut arrangements: Vec<Vec<&str>> = Vec::new();
    
    for stat in all_stats.lines() {
        let stat: Vec<&str> = stat.split_ascii_whitespace().collect();
        people.insert(stat[0]);

        stats
            .entry(stat[0])
            .and_modify(|neighbour: &mut HashMap<&str, i32>| {
                neighbour
                    .insert(
                        stat[10],
                        (||
                            if stat[2] == "gain" {
                                stat[3].parse().unwrap()
                            } else {
                                0 - stat[3].parse::<i32>().unwrap()
                            }
                        ) ()
                    );
            })
            .or_insert((||
                if stat[2] == "gain" {
                    HashMap::from([(stat[10], stat[3].parse().unwrap())])
                } else {
                    HashMap::from([(stat[10], 0 - stat[3].parse::<i32>().unwrap())])
                }) ()
            );
        }
        
    generate_arrangements(&mut arrangements, Vec::new(), people);

    let mut happiness: i32 = MIN;
    let mut cur_happiness: i32;
    for arrangement in arrangements {
        cur_happiness = 0;

        for pair in arrangement.windows(2) {
            cur_happiness += stats[pair[0]][pair[1]];
            cur_happiness += stats[pair[1]][pair[0]];
        }
        
        cur_happiness += stats[arrangement[0]][arrangement[arrangement.len() - 1]];
        cur_happiness += stats[arrangement[arrangement.len() - 1]][arrangement[0]];

        happiness = happiness.max(cur_happiness);
    }

    println!("{}", happiness);
}

fn generate_arrangements<'a, 'b: 'a, 'c: 'b>(
    arrangements: &mut Vec<Vec<&'a str>>,
    seated: Vec<&'b str>,
    left: HashSet<&'c str>
) {
    if left.len() == 0 {
        arrangements.push(seated);
        return;
    }

    for person in &left {
        let mut left: HashSet<&str> = left.clone();
        let mut seated: Vec<&str> = seated.clone();
        seated.push(*person);
        left.remove(*person);
        generate_arrangements(arrangements, seated, left.clone());
    }
}
