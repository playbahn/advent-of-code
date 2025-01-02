const INPUT: &str = "input/day-20.txt";
const EDGE: usize = 141;
const START: Point = Point { x: 17, y: 75 };
const END: Point = Point { x: 7, y: 93 };
const SAVEDLB: usize = 99;

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

fn count_cheats(start: Point, map: &[[Tile; EDGE]; EDGE]) -> usize {
    (1..21).fold(0, |count_before_dist, dist| {
        count_before_dist
            + (0..dist + 1)
                .flat_map(|x| {
                    let mut at_dist: Vec<(isize, isize)> = vec![
                        (start.x as isize - x, start.y as isize + dist - x),
                        (start.x as isize + x, start.y as isize - dist + x),
                        (start.x as isize + x, start.y as isize + dist - x),
                        (start.x as isize - x, start.y as isize - dist + x),
                    ];

                    if x == 0 || x == dist {
                        unsafe { at_dist.set_len(2) }
                    }

                    at_dist
                })
                .filter(|(x, y)| {
                    (-1 < *x && *x < EDGE as isize)
                        && (-1 < *y && *y < EDGE as isize)
                        && (map[*x as usize][*y as usize].time)
                            .checked_sub(map[start.x][start.y].time)
                            .is_some_and(|diff| diff > SAVEDLB + dist as usize)
                })
                .count()
    })
}

fn main() {
    let start = std::time::Instant::now();
    let mut map: [[Tile; EDGE]; EDGE] = [[Tile::new('\0', 0); EDGE]; EDGE];

    std::fs::read_to_string(INPUT)
        .unwrap()
        .lines()
        .enumerate()
        .for_each(|(y, row)| row.char_indices().for_each(|(x, obj)| map[x][y].obj = obj));

    let mut go = Left;
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

            for opps in opposites {
                if map[opps.0.x][opps.0.y].obj != '#'
                    && map[opps.1.x][opps.1.y].obj != '#'
                    && map[opps.0.x][opps.0.y]
                        .time
                        .abs_diff(map[opps.1.x][opps.1.y].time)
                        > SAVEDLB + 2
                {
                    part1 += 1;
                }
            }
        }
    }


    let part2 = path
        .iter()
        .take(path.len() - (SAVEDLB + 1) - 2)
        .fold(0, |count, start| count + count_cheats(*start, &map));

    println!("{}s", start.elapsed().as_secs_f64());
    println!("part1: {part1}\npart2: {part2}");
}
