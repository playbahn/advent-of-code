use std::collections::{HashSet, VecDeque};
use std::io::Write;
use std::time::Duration;

const INPUT: &str = "input/day-15-eg-small.txt";
const EDGE: usize = 7;
const STEP_RATE: u64 = 250;

#[derive(PartialEq, Clone, Copy)]
enum WallState {
    Hit,
    NotHit,
}

use WallState::*;

impl std::fmt::Display for WallState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NotHit => write!(f, "\x1B[;1m#\x1B[m"),
            Hit => write!(f, "\x1B[;1;31m#\x1B[m"),
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
enum BoxEdge {
    Left,
    Right,
}

use BoxEdge::*;

impl std::fmt::Display for BoxEdge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Left => write!(f, "["),
            Right => write!(f, "]"),
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
enum BoxState {
    Blocked(BoxEdge),
    Moved(BoxEdge),
    Idle(BoxEdge),
}

use BoxState::*;

impl std::fmt::Display for BoxState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Blocked(box_edge) => write!(f, "\x1B[;1;31m{box_edge}\x1b[m"),
            Moved(box_edge) => write!(f, "\x1B[;1;32m{box_edge}\x1b[m"),
            Idle(box_edge) => write!(f, "\x1B[1;33m{box_edge}\x1b[m"),
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
enum Tile {
    FreeSpace,
    Robot,
    Wall(WallState),
    Box(BoxState),
}

use Tile::*;

impl Tile {
    fn idle(&mut self) {
        *self = match self {
            Wall(Hit) => Wall(NotHit),
            Box(Blocked(box_edge) | Moved(box_edge)) => Box(Idle(*box_edge)),
            _ => *self,
        }
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FreeSpace => write!(f, "\x1B[;2;37m.\x1B[m"),
            Robot => write!(f, "\x1B[;1;36m@\x1B[m"),
            Wall(wall) => write!(f, "{wall}"),
            Box(state) => write!(f, "{state}"),
        }
    }
}

enum CanShift {
    Yes(VecDeque<(Point, Point)>),
    No(Vec<Point>),
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn take_step(&self, step: char, map: &[[Tile; EDGE]; EDGE * 2]) -> CanShift {
        let mut shifts: VecDeque<(Self, Self)> = VecDeque::new();

        let mut blocked: Vec<Self> = Vec::new();
        let mut visited: HashSet<Self> = HashSet::new();

        let next = match step {
            '^' => Self::new(self.x, self.y - 1),
            'v' => Self::new(self.x, self.y + 1),
            '<' => Self::new(self.x - 1, self.y),
            '>' => Self::new(self.x + 1, self.y),
            // does not matter what we are returning here
            ' ' => return CanShift::Yes(shifts),
            _ => panic!(),
        };

        if map[next.x][next.y] == FreeSpace {
            shifts.push_back((*self, next));
        } else {
            next.visit(step, &mut blocked, &mut shifts, &mut visited, map);
            shifts.push_front((*self, next));
        }

        if blocked.is_empty() {
            CanShift::Yes(shifts)
        } else {
            blocked.extend(shifts.into_iter().map(|(from, _)| from));
            CanShift::No(blocked)
        }
    }

    fn visit(
        &self,
        step: char,
        blocks: &mut Vec<Self>,
        shifts: &mut VecDeque<(Self, Self)>,
        visited: &mut HashSet<Self>,
        map: &[[Tile; EDGE]; EDGE * 2],
    ) {
        if map[self.x][self.y] == Wall(NotHit) {
            blocks.push(*self);
            return;
        } else if !visited.insert(*self) {
            return;
        }

        let (next, can_shift) = match step {
            '^' => (
                Self::new(self.x, self.y - 1),
                map[self.x][self.y - 1] == FreeSpace,
            ),
            'v' => (
                Self::new(self.x, self.y + 1),
                map[self.x][self.y + 1] == FreeSpace,
            ),
            '<' => (
                Self::new(self.x - 1, self.y),
                map[self.x - 1][self.y] == FreeSpace,
            ),
            '>' => (
                Self::new(self.x + 1, self.y),
                map[self.x + 1][self.y] == FreeSpace,
            ),
            _ => panic!(),
        };

        if can_shift {
            shifts.push_back((*self, next));
        } else {
            next.visit(step, blocks, shifts, visited, map);
            shifts.push_front((*self, next));
        }

        match (map[self.x][self.y], step) {
            (Box(Idle(Left)), '^' | 'v') => {
                Self::new(self.x + 1, self.y).visit(step, blocks, shifts, visited, map);
            }
            (Box(Idle(Right)), '^' | 'v') => {
                Self::new(self.x - 1, self.y).visit(step, blocks, shifts, visited, map);
            }
            _ => (),
        }
    }
}

fn main() {
    ctrlc::set_handler(|| {
        println!("\x1B[{}H\x1B[?25h\x1B[m", EDGE + 3);
        std::process::exit(0);
    })
    .unwrap();

    let input = std::fs::read_to_string(INPUT).unwrap();
    let mut wide_map: [[Tile; EDGE]; EDGE * 2] = [[FreeSpace; EDGE]; EDGE * 2];

    let mut robot: Point = Point::new(usize::MAX, usize::MAX);

    input
        .lines()
        .take_while(|line| !line.is_empty())
        .enumerate()
        .for_each(|(y, line)| {
            line.char_indices().for_each(|(x, tile)| {
                (wide_map[x * 2][y], wide_map[x * 2 + 1][y]) = match tile {
                    'O' => (Box(Idle(Left)), Box(Idle(Right))),
                    '@' => {
                        robot = Point::new(x << 1, y);
                        (Robot, FreeSpace)
                    }
                    '#' => (Wall(NotHit), Wall(NotHit)),
                    '.' => (FreeSpace, FreeSpace),
                    _ => panic!(),
                };
            })
        });

    let mut steps: VecDeque<char> = input
        .lines()
        .skip_while(|line| line.starts_with(['#', '\n']))
        .flat_map(|steps| steps.chars())
        .collect();

    const TITLE: &str = "Advent of Code 2024: Day 15: Warehouse Woes (Part Two)";
    let steps_total: String = steps.len().to_string();
    let steps_len_s: usize = steps_total.len();
    let dim_ub =
        (TITLE.len().max(EDGE << 1) - "Steps: ".len() - ((steps_total.len() << 1) + 3)) >> 1;

    // solely for animating the "steps window"
    (0..dim_ub).for_each(|_| {
        steps.push_front(' ');
        steps.push_back(' ');
    });

    let mut step_count = 0usize;

    let mut steps_window = |step: (usize, &char)| {
        if step.1 != &' ' {
            step_count += 1;
        }

        print!("\x1B[3;8H\x1B[;2;36m");

        (1..dim_ub)
            .rev()
            .for_each(|n| print!("{}", steps[step.0 - n]));

        print!("\x1B[;1;96m{}\x1B[;2;36m", step.1);

        (1..dim_ub).for_each(|n| print!("{}", steps[step.0 + n]));

        println!(" \x1B[;1;96m({step_count:>steps_len_s$}/{steps_total})\x1B[m");

        // std::thread::sleep(Duration::from_millis(STEP_RATE));
    };

    let mut updates: HashSet<Point> = HashSet::new();

    let steps = steps
        .iter()
        .enumerate()
        .skip(dim_ub - 1)
        .take(steps.len() - (dim_ub << 1) + 2);

    let mut gps_acc = 0usize;

    (1..EDGE - 1).for_each(|y| {
        (2..2 * (EDGE - 1)).for_each(|x| {
            if let Box(Idle(Left)) = wide_map[x][y] {
                gps_acc += x + 100 * y;
            }
        })
    });

    println!("\x1B[?25l\x1B[2J\x1B[H\x1B[;95m{TITLE}");
    println!("\x1B[;93mGPS accumulate: {gps_acc}\x1b[m");
    println!("\x1B[;36mSteps: ");

    // Initial warehouse map
    (0..EDGE).for_each(|y| {
        (0..EDGE << 1).for_each(|x| print!("{}", wide_map[x][y]));
        println!();
    });

    // let mut user_input: String = String::new();

    for step in steps {
        // update the cells from prev iteration for current iteration
        updates
            .drain()
            .for_each(|pt| print!("\x1B[{};{}H{}", 4 + pt.y, 1 + pt.x, wide_map[pt.x][pt.y]));

        match robot.take_step(*step.1, &wide_map) {
            CanShift::Yes(mut shift_coords) => {
                while let Some((from, to)) = shift_coords.pop_back() {
                    updates.insert(from);
                    updates.insert(to);
                    wide_map[to.x][to.y] = match wide_map[from.x][from.y] {
                        Box(Idle(Left)) => {
                            gps_acc -= from.x + 100 * from.y;
                            gps_acc += to.x + 100 * to.y;
                            Box(Moved(Left))
                        }
                        Box(Idle(Right)) => Box(Moved(Right)),
                        _ => wide_map[from.x][from.y],
                    };

                    wide_map[from.x][from.y] = FreeSpace;
                    robot = to;
                }
            }
            CanShift::No(blocks) => {
                for block in blocks {
                    updates.insert(block);
                    wide_map[block.x][block.y] = match wide_map[block.x][block.y] {
                        Wall(NotHit) => Wall(Hit),
                        Box(Idle(box_edge)) => Box(Blocked(box_edge)),
                        tile => tile,
                    }
                }
            }
        }

        steps_window(step);

        // update the cells that were modified in just the current iteration
        updates.iter().for_each(|pt| {
            print!("\x1B[{};{}H{}", 4 + pt.y, 1 + pt.x, wide_map[pt.x][pt.y]);
            wide_map[pt.x][pt.y].idle();
        });

        print!("\x1B[2;17H\x1B[;93m{gps_acc}\x1B[m");
        // flush prev & current iteration updates, and new gpc_acc
        std::io::stdout().flush().unwrap();

        // for taking screenshots
        // std::io::stdin()
        //     .read_line(&mut user_input)
        //     .expect("Failed to read line");

        std::thread::sleep(Duration::from_millis(STEP_RATE));
    }

    print!("\x1B[{}H\x1B[?25h\x1B[m", EDGE + 4);
    std::io::stdout().flush().unwrap();
}
