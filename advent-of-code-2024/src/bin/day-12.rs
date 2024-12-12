use std::collections::{HashMap, HashSet};

type Coords = (usize, usize);

struct Region {
    area: u32,
    perimeter: u32,
}

impl std::ops::AddAssign for Region {
    fn add_assign(&mut self, rhs: Self) {
        self.area += rhs.area;
        self.perimeter += rhs.perimeter;
    }
}

const INPUT: &str = "input/day-12.txt";
const UB: usize = 140;
const ASCII_A: u8 = 65;
const ASCII_Z: u8 = 90;

fn region_ap(
    start: Coords,
    unvisited: &mut HashSet<Coords>,
    map: &[[char; UB]; UB],
    plant: char,
) -> Region {
    let mut region = Region {
        area: 1,
        perimeter: 4,
    };

    unvisited.remove(&start);

    if let Some(left) = start.0.checked_sub(1) {
        if map[left][start.1] == plant {
            region.perimeter -= 1;
            if unvisited.contains(&(left, start.1)) {
                region += region_ap((left, start.1), unvisited, map, plant);
            }
        }
    }

    if start.0 + 1 < UB && map[start.0 + 1][start.1] == plant {
        region.perimeter -= 1;
        if unvisited.contains(&(start.0 + 1, start.1)) {
            region += region_ap((start.0 + 1, start.1), unvisited, map, plant);
        }
    }

    if let Some(up) = start.1.checked_sub(1) {
        if map[start.0][up] == plant {
            region.perimeter -= 1;
            if unvisited.contains(&(start.0, up)) {
                region += region_ap((start.0, up), unvisited, map, plant);
            }
        }
    }

    if start.1 + 1 < UB && map[start.0][start.1 + 1] == plant {
        region.perimeter -= 1;
        if unvisited.contains(&(start.0, start.1 + 1)) {
            region += region_ap((start.0, start.1 + 1), unvisited, map, plant);
        }
    }

    region
}

fn main() {
    let mut map: [[char; UB]; UB] = [['\0'; UB]; UB];
    // to keep track of unvisited plots; `V` = `HashSet<_>` just for easy deletion.
    let mut unvisited: HashMap<char, HashSet<Coords>> =
        HashMap::from_iter((ASCII_A..ASCII_Z + 1).map(|dec| (dec as char, HashSet::new())));

    std::fs::read_to_string(INPUT)
        .unwrap()
        .lines()
        .enumerate()
        .for_each(|(y, isle)| {
            isle.char_indices().for_each(|(x, plant)| {
                map[x][y] = plant;
                unvisited.get_mut(&plant).unwrap().insert((x, y));
            })
        });

    let mut region: Region;
    let mut part1 = 0u32;
    let mut plant: char;

    for y in 0..UB {
        for x in 0..UB {
            plant = map[x][y];
            if unvisited[&plant].contains(&(x, y)) {
                // different region than previous `(x, y)`
                region = region_ap((x, y), unvisited.get_mut(&plant).unwrap(), &map, plant);
                part1 += region.area * region.perimeter;
            }
        }
    }

    println!("part1: {part1}")
}
