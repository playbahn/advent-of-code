use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Facing {
    North,
    South,
    East,
    West,
}

type Coords = (usize, usize);

const INPUT: &str = "input/day-06.txt";
const UB: usize = 130;
const GUARD_INIT: Coords = (61, 85);

fn get_obs(guard: Coords, facing: Facing, obstacles: &[Coords]) -> Option<&Coords> {
    match facing {
        Facing::North => obstacles
            .iter()
            .filter(|obs| obs.0 == guard.0 && obs.1 < guard.1)
            .max_by(|obs1, obs2| obs1.1.cmp(&obs2.1)),
        Facing::South => obstacles
            .iter()
            .filter(|obs| obs.0 == guard.0 && obs.1 > guard.1)
            .min_by(|obs1, obs2| obs1.1.cmp(&obs2.1)),
        Facing::East => obstacles
            .iter()
            .filter(|obs| obs.1 == guard.1 && obs.0 > guard.0)
            .min_by(|obs1, obs2| obs1.0.cmp(&obs2.0)),
        Facing::West => obstacles
            .iter()
            .filter(|obs| obs.1 == guard.1 && obs.0 < guard.0)
            .max_by(|obs1, obs2| obs1.0.cmp(&obs2.0)),
    }
}

fn simulate_path(obstacles: &[Coords], unique: &mut HashSet<Coords>) {
    let mut guard = GUARD_INIT;
    let mut facing = Facing::North;

    loop {
        match facing {
            Facing::North => {
                if let Some(obs) = get_obs(guard, facing, obstacles) {
                    unique.extend((obs.1 + 1..guard.1).map(|y| (guard.0, y)));
                    facing = Facing::East;
                    guard.1 = obs.1 + 1;
                } else {
                    unique.extend((0..guard.1).map(|y| (guard.0, y)));
                    break;
                }
            }
            Facing::South => {
                if let Some(obs) = get_obs(guard, facing, obstacles) {
                    unique.extend((guard.1 + 1..obs.1).map(|y| (guard.0, y)));
                    facing = Facing::West;
                    guard.1 = obs.1 - 1;
                } else {
                    unique.extend((guard.1 + 1..UB).map(|y| (guard.0, y)));
                    break;
                }
            }
            Facing::East => {
                if let Some(obs) = get_obs(guard, facing, obstacles) {
                    unique.extend((guard.0 + 1..obs.0).map(|x| (x, guard.1)));
                    facing = Facing::South;
                    guard.0 = obs.0 - 1;
                } else {
                    unique.extend((guard.0 + 1..UB).map(|x| (x, guard.1)));
                    break;
                }
            }
            Facing::West => {
                if let Some(obs) = get_obs(guard, facing, obstacles) {
                    unique.extend((obs.0 + 1..guard.0).map(|x| (x, guard.1)));
                    facing = Facing::North;
                    guard.0 = obs.0 + 1;
                } else {
                    unique.extend((0..guard.0).map(|x| (x, guard.1)));
                    break;
                }
            }
        }
    }
}

fn put_guard_to_loop(obstacles: &[Coords]) -> bool {
    let mut guard = GUARD_INIT;
    let mut facing = Facing::North;
    let mut faced: HashSet<(Coords, Facing)> = HashSet::new();
    
    loop {
        match facing {
            Facing::North => {
                if let Some(obs) = get_obs(guard, facing, obstacles) {
                    if !faced.insert((*obs, facing)) {
                        break true;
                    }
                    facing = Facing::East;
                    guard.1 = obs.1 + 1;
                } else {
                    break false;
                }
            }
            Facing::South => {
                if let Some(obs) = get_obs(guard, facing, obstacles) {
                    if !faced.insert((*obs, facing)) {
                        break true;
                    }
                    facing = Facing::West;
                    guard.1 = obs.1 - 1;
                } else {
                    break false;
                }
            }
            Facing::East => {
                if let Some(obs) = get_obs(guard, facing, obstacles) {
                    if !faced.insert((*obs, facing)) {
                        break true;
                    }
                    facing = Facing::South;
                    guard.0 = obs.0 - 1;
                } else {
                    break false;
                }
            }
            Facing::West => {
                if let Some(obs) = get_obs(guard, facing, obstacles) {
                    if !faced.insert((*obs, facing)) {
                        break true;
                    }
                    facing = Facing::North;
                    guard.0 = obs.0 + 1;
                } else {
                    break false;
                }
            }
        }
    }
}

fn main() {
    let mut obstacles: Vec<Coords> = Vec::from_iter(
        std::fs::read_to_string(INPUT)
            .unwrap()
            .match_indices('#')
            .map(|(idx, _)| (idx % (UB + 1), idx / (UB + 1))), // UB + 1 `\n`
    );

    // part 1
    let mut uniques: HashSet<Coords> = HashSet::from([GUARD_INIT]);
    simulate_path(&obstacles, &mut uniques);
    println!("part1: {}", uniques.len());

    // part 2
    uniques.remove(&GUARD_INIT);
    let mut new_obs_pos = 0u16;
    for unique in uniques {
        obstacles.push(unique);
        if put_guard_to_loop(&obstacles) {
            new_obs_pos += 1;
        }
        obstacles.pop();
    }
    println!("part2: {}", new_obs_pos);
}
