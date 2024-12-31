const INPUT: &str = "input/day-20.txt";
const GO: Go = Left;
const EDGE: usize = 141;
const START: Point = Point { x: 17, y: 75 };
const END: Point = Point { x: 7, y: 93 };
const SAVEDLB1: usize = 99;
const SAVEDLB2: usize = 99;

#[derive(PartialEq, Clone, Copy)]
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

#[derive(Clone, Copy, PartialEq)]
enum Go {
    Up,
    Down,
    Left,
    Right,
}

use Go::*;

fn count_cheats(s: Point, map: &[[Tile; EDGE]; EDGE]) -> usize {
    let mut count = 0usize;

    (1..21).for_each(|dist| {
        count += (0..dist + 1)
            .flat_map(|x| {
                let mut at_manhattan_dist: Vec<(isize, isize)> = vec![
                    (s.x as isize - x, s.y as isize + dist - x),
                    (s.x as isize + x, s.y as isize - dist + x),
                ];

                if x != 0 && x != dist {
                    at_manhattan_dist.push((s.x as isize + x, s.y as isize + dist - x));
                    at_manhattan_dist.push((s.x as isize - x, s.y as isize - dist + x));
                }

                at_manhattan_dist
            })
            .filter(|(x, y)| {
                (-1 < *x && *x < EDGE as isize)
                    && (-1 < *y && *y < EDGE as isize)
                    && map[*x as usize][*y as usize]
                        .time
                        .checked_sub(map[s.x][s.y].time)
                        .is_some_and(|diff| diff > SAVEDLB2 + dist as usize)
            })
            .count();
    });

    count
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

    let mut part1 = 0usize;

    for y in 1..EDGE - 1 {
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
                    && map[opp.0.x][opp.0.y]
                        .time
                        .abs_diff(map[opp.1.x][opp.1.y].time)
                        > SAVEDLB1 + 2
                {
                    part1 += 1;
                }
            }
        }
    }

    println!("part1: {part1}");

    // let mut saves = vec![];

    // path.iter()
    //     .take(path.len() - (SAVEDLB2 + 1) - 2)
    //     .for_each(|s| saves.extend(count_cheats(*s, &map)));

    // let mut unique = saves.clone();

    // unique.sort();
    // unique.dedup();

    // for saved in unique {
    //     let count = saves.iter().filter(|c_save| **c_save == saved).count();
    //     println!("There are {count} cheats that save {saved} picoseconds")
    // }

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
