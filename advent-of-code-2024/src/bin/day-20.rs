use std::cmp::Ordering::*;
use std::collections::{HashMap, HashSet};

// const INPUT: &str = "input/day-20-eg.txt";
// const GO: Go = Up;
// const EDGE: usize = 15;
// const START: Point = Point { x: 1, y: 3 };
// const END: Point = Point { x: 5, y: 7 };
// const SAVEDLB1: usize = 0;
// const SAVEDLB2: usize = 49;

const INPUT: &str = "input/day-20.txt";
const GO: Go = Left;
const EDGE: usize = 141;
const START: Point = Point { x: 17, y: 75 };
const END: Point = Point { x: 7, y: 93 };
const SAVEDLB1: usize = 99;
const SAVEDLB2: usize = 99;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Copy)]
struct Tile {
    obj: char,
    time: usize,
}

impl Tile {
    fn new(obj: char, time: usize) -> Self {
        Self { obj, time }
    }
}

#[derive(PartialEq, Eq, Hash)]
struct Cheat {
    start: Point,
    end: Point,
}

impl Cheat {
    fn new(start: Point, end: Point) -> Self {
        Self { start, end }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Go {
    Up,
    Down,
    Left,
    Right,
}

use Go::*;

fn get_neighbors(pt: &Point) -> Vec<Point> {
    let mut neighbors = vec![];

    if pt.x > 0 {
        neighbors.push(Point::new(pt.x - 1, pt.y));
    }

    if pt.x + 1 < EDGE {
        neighbors.push(Point::new(pt.x + 1, pt.y));
    }

    if pt.y > 0 {
        neighbors.push(Point::new(pt.x, pt.y - 1));
    }

    if pt.y + 1 < EDGE {
        neighbors.push(Point::new(pt.x, pt.y + 1));
    }

    neighbors
}

fn count_cheats(s: Point, map: &[[Tile; EDGE]; EDGE]) -> usize {
    let mut ends: HashMap<Point, usize> = HashMap::new();
    let mut reachable: HashMap<Point, usize> = HashMap::new();
    let mut outmost_cur: HashSet<Point> = HashSet::new();
    let mut outmost_next: HashSet<Point> = HashSet::new();

    outmost_cur.insert(s);

    for cur_time in 0..20 {
        if outmost_cur.is_empty() {
            break;
        }

        for outmost_pt in &outmost_cur {
            for n in get_neighbors(outmost_pt) {
                if !reachable.contains_key(&n) {
                    outmost_next.insert(n);
                }
            }
        }

        reachable.extend(outmost_cur.drain().map(|pt| (pt, cur_time)));

        outmost_cur.extend(outmost_next.drain());
    }

    reachable.remove(&s);

    for (wall, cheat_time) in &reachable {
        for f in get_neighbors(wall) {
            if map[f.x][f.y].obj != '#' {
                if let Some(diff) = map[f.x][f.y].time.checked_sub(map[s.x][s.y].time) {
                    if diff > SAVEDLB2 + cheat_time + 1 {
                        ends.entry(f)
                            .and_modify(|saved| *saved = (*saved).max(diff - (cheat_time + 1)))
                            .or_insert(diff - (cheat_time + 1));
                    }
                }
            }
        }
    }

    ends.len()
}

fn main() {
    let mut map: [[Tile; EDGE]; EDGE] = [[Tile::new('\0', 0); EDGE]; EDGE];

    std::fs::read_to_string(INPUT)
        .unwrap()
        .lines()
        .enumerate()
        .for_each(|(y, row)| row.char_indices().for_each(|(x, obj)| map[x][y].obj = obj));

    let mut go = GO;
    let mut time = 0usize;
    let mut cur = START;
    let mut path: Vec<Point> = vec![START];

    // populate "track" tiles' times
    'run: loop {
        match go {
            Up => {
                while map[cur.x][cur.y - 1].obj != '#' {
                    cur.y -= 1;
                    time += 1;
                    map[cur.x][cur.y].time = time;
                    path.push(cur);
                    if cur == END {
                        break 'run;
                    }
                }
            }
            Down => {
                while map[cur.x][cur.y + 1].obj != '#' {
                    cur.y += 1;
                    time += 1;
                    map[cur.x][cur.y].time = time;
                    path.push(cur);
                    if cur == END {
                        break 'run;
                    }
                }
            }
            Left => {
                while map[cur.x - 1][cur.y].obj != '#' {
                    cur.x -= 1;
                    time += 1;
                    map[cur.x][cur.y].time = time;
                    path.push(cur);
                    if cur == END {
                        break 'run;
                    }
                }
            }
            Right => {
                while map[cur.x + 1][cur.y].obj != '#' {
                    cur.x += 1;
                    time += 1;
                    map[cur.x][cur.y].time = time;
                    path.push(cur);
                    if cur == END {
                        break 'run;
                    }
                }
            }
        } // match go

        match (
            go,
            map[cur.x][cur.y - 1].obj,
            map[cur.x][cur.y + 1].obj,
            map[cur.x - 1][cur.y].obj,
            map[cur.x + 1][cur.y].obj,
        ) {
            (Left | Right, '#', '.', _, _) => go = Down,
            (Left | Right, '.', '#', _, _) => go = Up,
            (Up | Down, _, _, '#', '.') => go = Right,
            (Up | Down, _, _, '.', '#') => go = Left,
            _ => panic!(),
        }
    } // 'run: loop

    let mut cheats: HashSet<Cheat> = HashSet::new();

    '_part1: for y in 1..EDGE - 1 {
        for x in 1..EDGE - 1 {
            if map[x][y].obj != '#' {
                continue;
            }

            let opposites = [
                (Point::new(x - 1, y), Point::new(x + 1, y)),
                (Point::new(x, y - 1), Point::new(x, y + 1)),
            ];

            for opp in opposites {
                if map[opp.0.x][opp.0.y].obj != '#'
                    && map[opp.1.x][opp.1.y].obj != '#'
                    && (map[opp.0.x][opp.0.y].time).abs_diff(map[opp.1.x][opp.1.y].time)
                        > SAVEDLB1 + 2
                {
                    match map[opp.0.x][opp.0.y].time.cmp(&map[opp.1.x][opp.1.y].time) {
                        Less => cheats.insert(Cheat::new(opp.0, opp.1)),
                        Greater => cheats.insert(Cheat::new(opp.1, opp.0)),
                        Equal => panic!(),
                    };
                }
            }
        }
    }

    println!("part1: {}", cheats.len());

    let part2 = path
        .iter()
        .take(path.len() - (SAVEDLB2 + 1) - 2)
        .fold(0, |count, s| count + count_cheats(*s, &map));

    println!("part2: {part2}");
}

impl std::fmt::Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.obj, self.time)
    }
}

impl std::fmt::Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl std::fmt::Debug for Cheat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} -> {:?}", self.start, self.end)
    }
}
