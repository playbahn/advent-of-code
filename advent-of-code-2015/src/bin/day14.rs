use std::fs;

#[derive(Debug)]
struct Reindeer {
    dist: u32,
    points: u32,
    speed: u8,
    flown: u16,
    fly_time: u8,
    rested: u16,
    rest_time: u8,
    state: State,
}

#[derive(Debug)]
enum State {
    Active,
    Inactive,
}
use State::*;

fn main() {
    let stats: String = fs::read_to_string("input/day14.txt").unwrap();
    let mut reindeers: Vec<Reindeer> = Vec::new();

    for stat in stats.lines() {
        let stat: Vec<&str> = stat.split_ascii_whitespace().collect();

        let reindeer: Reindeer = Reindeer {
            dist: 0,
            points: 0,
            speed: stat[3].parse().unwrap(),
            flown: 0,
            fly_time: stat[6].parse().unwrap(),
            rested: 0,
            rest_time: stat[13].parse().unwrap(),
            state: Active,
        };

        reindeers.push(reindeer);
    }

    let mut max_dist: u32;
    for _ in 0..2503u16 {
        max_dist = 0;

        for reindeer in &mut reindeers {
            match reindeer.state {
                Active => {
                    reindeer.dist += reindeer.speed as u32;
                    reindeer.flown += 1;
                    if reindeer.flown == reindeer.fly_time as u16 {
                        reindeer.flown = 0;
                        reindeer.state = Inactive;
                    }
                },
                Inactive => {
                    reindeer.rested += 1;
                    if reindeer.rested == reindeer.rest_time as u16 {
                        reindeer.rested = 0;
                        reindeer.state = Active;
                    }
                },
            }

            max_dist = max_dist.max(reindeer.dist);
        }

        for reindeer in &mut reindeers {
            if reindeer.dist == max_dist {
                reindeer.points += 1;
            }
        }
    }

    let mut most_flown: u32 = 0;
    let mut most_ponints: u32 = 0;

    for reindeer in reindeers {
        most_flown = most_flown.max(reindeer.dist);
        most_ponints = most_ponints.max(reindeer.points);
    }

    println!("{}", most_flown);
    println!("{}", most_ponints);
}
