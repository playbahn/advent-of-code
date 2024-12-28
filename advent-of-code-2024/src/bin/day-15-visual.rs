use std::collections::{HashSet, VecDeque};
use std::io::Write;
use std::time::Duration;

const INPUT: &str = "input/day-15-eg-large.txt";
const EDGE: usize = 10;
const STEP_RATE: u64 = 0;

#[derive(PartialEq, Clone, Copy)]
enum Wall {
    Hit,
    Idle,
}

impl std::fmt::Display for Wall {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Wall::Idle => write!(f, "\x1B[;1m#\x1B[m"),
            Wall::Hit => write!(f, "\x1B[;1;31m#\x1B[m"),
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
enum BoxEdge {
    Left,
    Right,
}

impl std::fmt::Display for BoxEdge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BoxEdge::Left => write!(f, "["),
            BoxEdge::Right => write!(f, "]"),
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
enum Box {
    Blocked(BoxEdge),
    Moved(BoxEdge),
    Idle(BoxEdge),
}

impl std::fmt::Display for Box {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Box::Blocked(box_edge) => write!(f, "\x1B[;1;31m{box_edge}\x1b[m"),
            Box::Moved(box_edge) => write!(f, "\x1B[;1;32m{box_edge}\x1b[m"),
            Box::Idle(box_edge) => write!(f, "\x1B[1;33m{box_edge}\x1b[m"),
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
enum Tile {
    FreeSpace,
    Robot,
    Wall(Wall),
    Box(Box),
}

impl Tile {
    fn idle(&mut self) {
        match self {
            Tile::Wall(Wall::Hit) => *self = Tile::Wall(Wall::Idle),
            Tile::Box(Box::Blocked(box_edge) | Box::Moved(box_edge)) => {
                *self = Tile::Box(Box::Idle(*box_edge))
            }
            _ => (),
        }
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::FreeSpace => write!(f, "\x1B[;2;37m.\x1B[m"),
            Tile::Robot => write!(f, "\x1B[;1;36m@\x1B[m"),
            Tile::Wall(wall) => write!(f, "{wall}"),
            Tile::Box(state) => write!(f, "{state}"),
        }
    }
}

enum CanShift {
    Yes(VecDeque<(Point, Point)>),
    No(HashSet<Point>),
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

        let mut blocked = false;
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

        if map[next.x][next.y] == Tile::FreeSpace {
            shifts.push_back((*self, next));
        } else {
            next.visit(step, &mut blocked, &mut shifts, &mut visited, map);
            shifts.push_front((*self, next));
        }

        if blocked {
            CanShift::No(HashSet::from_iter(
                shifts.into_iter().flat_map(|(from, to)| vec![from, to]),
            ))
        } else {
            CanShift::Yes(shifts)
        }
    }

    fn visit(
        &self,
        step: char,
        blocked: &mut bool,
        shifts: &mut VecDeque<(Self, Self)>,
        visited: &mut HashSet<Self>,
        map: &[[Tile; EDGE]; EDGE * 2],
    ) {
        if map[self.x][self.y] == Tile::Wall(Wall::Idle) {
            *blocked = true;
            return;
        } else if !visited.insert(*self) {
            return;
        }

        let (next, can_shift) = match step {
            '^' => (
                Self::new(self.x, self.y - 1),
                map[self.x][self.y - 1] == Tile::FreeSpace,
            ),
            'v' => (
                Self::new(self.x, self.y + 1),
                map[self.x][self.y + 1] == Tile::FreeSpace,
            ),
            '<' => (
                Self::new(self.x - 1, self.y),
                map[self.x - 1][self.y] == Tile::FreeSpace,
            ),
            '>' => (
                Self::new(self.x + 1, self.y),
                map[self.x + 1][self.y] == Tile::FreeSpace,
            ),
            _ => panic!(),
        };

        if can_shift {
            shifts.push_back((*self, next));
        } else {
            next.visit(step, blocked, shifts, visited, map);
            shifts.push_front((*self, next));
        }

        match (map[self.x][self.y], step) {
            (Tile::Box(Box::Idle(BoxEdge::Left)), '^' | 'v') => {
                Self::new(self.x + 1, self.y).visit(step, blocked, shifts, visited, map);
            }
            (Tile::Box(Box::Idle(BoxEdge::Right)), '^' | 'v') => {
                Self::new(self.x - 1, self.y).visit(step, blocked, shifts, visited, map);
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
    let mut wide_map: [[Tile; EDGE]; EDGE * 2] = [[Tile::FreeSpace; EDGE]; EDGE * 2];

    let mut robot: Point = Point::new(0, 0);

    input
        .lines()
        .take_while(|line| !line.is_empty())
        .enumerate()
        .for_each(|(y, line)| {
            line.char_indices().for_each(|(x, tile)| {
                (wide_map[x * 2][y], wide_map[x * 2 + 1][y]) = match tile {
                    'O' => (
                        Tile::Box(Box::Idle(BoxEdge::Left)),
                        Tile::Box(Box::Idle(BoxEdge::Right)),
                    ),
                    '@' => {
                        robot = Point::new(x << 1, y);
                        (Tile::Robot, Tile::FreeSpace)
                    }
                    '#' => (Tile::Wall(Wall::Idle), Tile::Wall(Wall::Idle)),
                    '.' => (Tile::FreeSpace, Tile::FreeSpace),
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

        println!(
            " \x1B[;1;96m({:>steps_len_s$}/{})\x1B[m",
            step_count, steps_total
        );

        std::thread::sleep(Duration::from_millis(STEP_RATE));
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
            if let Tile::Box(Box::Idle(BoxEdge::Left)) = wide_map[x][y] {
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

    let mut user_input: String = String::new();

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
                        Tile::Box(Box::Idle(BoxEdge::Left)) => {
                            gps_acc -= from.x + 100 * from.y;
                            gps_acc += to.x + 100 * to.y;
                            Tile::Box(Box::Moved(BoxEdge::Left))
                        }
                        Tile::Box(Box::Idle(BoxEdge::Right)) => {
                            Tile::Box(Box::Moved(BoxEdge::Right))
                        }
                        _ => wide_map[from.x][from.y],
                    };

                    wide_map[from.x][from.y] = Tile::FreeSpace;
                    robot = to;
                }
            }
            CanShift::No(blocks) => {
                for blocked in blocks {
                    updates.insert(blocked);
                    wide_map[blocked.x][blocked.y] = match wide_map[blocked.x][blocked.y] {
                        Tile::Wall(Wall::Idle) => Tile::Wall(Wall::Hit),
                        Tile::Box(Box::Idle(box_edge)) => Tile::Box(Box::Blocked(box_edge)),
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
        // flush prev iteration + current iteration updates
        std::io::stdout().flush().unwrap();

        // for taking screenshots
        std::io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        std::thread::sleep(Duration::from_millis(STEP_RATE));
    }

    print!("\x1B[{}H\x1B[?25h\x1B[m", EDGE + 4);
    std::io::stdout().flush().unwrap();
}
