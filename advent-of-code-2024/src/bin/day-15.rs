use std::collections::{HashSet, VecDeque};

const INPUT: &str = "input/day-15.txt";
const EDGE: usize = 50;
const ROBOT: Point = Point { x: 24, y: 24 };

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl std::fmt::Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Point {
    const LB: Self = Self { x: 0, y: 0 };
    const UB: Self = Self { x: EDGE, y: EDGE };

    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    /// Returns `Some(coords)` if `object` comes before `bounds`, otherwise returns
    /// `None`. If `bounds` is `None`, `Coords::UB` and `Coords::LB` are used
    /// instead, depending on `step`.
    ///
    /// # Panics
    ///
    /// Panics if `step` isn't any one of `'^'`, `'v'`, `'<'`, `'>'`.
    fn get_obj(
        &self,
        obj: char,
        within: Option<Self>,
        step: char,
        map: &[[char; EDGE]; EDGE],
    ) -> Option<Self> {
        match step {
            '^' => (within.unwrap_or(Self::LB).y..self.y)
                .filter_map(|y| (map[self.x][y] == obj).then_some(Self::new(self.x, y)))
                .max_by(|obja, objb| obja.y.cmp(&objb.y)),
            'v' => (self.y + 1..within.unwrap_or(Self::UB).y)
                .filter_map(|y| (map[self.x][y] == obj).then_some(Self::new(self.x, y)))
                .min_by(|obja, objb| obja.y.cmp(&objb.y)),
            '<' => (within.unwrap_or(Self::LB).x..self.x)
                .filter_map(|x| (map[x][self.y] == obj).then_some(Self::new(x, self.y)))
                .max_by(|obja, objb| obja.x.cmp(&objb.x)),
            '>' => (self.x + 1..within.unwrap_or(Self::UB).x)
                .filter_map(|x| (map[x][self.y] == obj).then_some(Self::new(x, self.y)))
                .min_by(|obja, objb| obja.x.cmp(&objb.x)),
            _ => panic!(),
        }
    }

    fn take_step(&self, step: char, map: &[[char; EDGE]; EDGE * 2]) -> VecDeque<(Point, Point)> {
        let mut shifts: VecDeque<(Self, Self)> = VecDeque::new();
        let mut blocked = false;
        let mut visited: HashSet<Self> = HashSet::new();

        let next = match step {
            '^' => Self::new(self.x, self.y - 1),
            'v' => Self::new(self.x, self.y + 1),
            '<' => Self::new(self.x - 1, self.y),
            '>' => Self::new(self.x + 1, self.y),
            _ => panic!(),
        };

        if map[next.x][next.y] == '.' {
            shifts.push_back((*self, next));
        } else {
            next.visit(step, &mut blocked, &mut shifts, &mut visited, map);
            if !blocked {
                shifts.push_front((*self, next));
            }
        }

        shifts
    }

    fn visit(
        &self,
        step: char,
        blocked: &mut bool,
        shifts: &mut VecDeque<(Point, Point)>,
        visited: &mut HashSet<Point>,
        map: &[[char; EDGE]; EDGE * 2],
    ) {
        if *blocked || map[self.x][self.y] == '#' {
            shifts.clear();
            *blocked = true;
            return;
        } else if !visited.insert(*self) {
            return;
        }

        let (next, can_shift) = match step {
            '^' => (
                Point::new(self.x, self.y - 1),
                map[self.x][self.y - 1] == '.',
            ),
            'v' => (
                Point::new(self.x, self.y + 1),
                map[self.x][self.y + 1] == '.',
            ),
            '<' => (
                Point::new(self.x - 1, self.y),
                map[self.x - 1][self.y] == '.',
            ),
            '>' => (
                Point::new(self.x + 1, self.y),
                map[self.x + 1][self.y] == '.',
            ),
            _ => panic!(),
        };

        if can_shift {
            shifts.push_back((*self, next));
        } else {
            next.visit(step, blocked, shifts, visited, map);
            if *blocked {
                return;
            } else {
                shifts.push_front((*self, next));
            }
        }

        match (map[self.x][self.y], step) {
            ('[', '^' | 'v') => {
                Point::new(self.x + 1, self.y).visit(step, blocked, shifts, visited, map);
            }
            (']', '^' | 'v') => {
                Point::new(self.x - 1, self.y).visit(step, blocked, shifts, visited, map);
            }
            _ => (),
        }
    }
}

fn main() {
    let input = std::fs::read_to_string(INPUT).unwrap();
    // part 1
    let mut map: [[char; EDGE]; EDGE] = [['\0'; EDGE]; EDGE];
    // part 2
    let mut wide_map: [[char; EDGE]; EDGE * 2] = [['\0'; EDGE]; EDGE * 2];

    input
        .lines()
        .take_while(|line| !line.is_empty())
        .enumerate()
        .for_each(|(y, line)| {
            line.char_indices().for_each(|(x, obj)| {
                map[x][y] = obj;
                (wide_map[x * 2][y], wide_map[x * 2 + 1][y]) =
                    if obj == 'O' { ('[', ']') } else { (obj, obj) };
            })
        });

    wide_map[ROBOT.x * 2 + 1][ROBOT.y] = '.';

    let steps = input
        .lines()
        .skip_while(|line| !line.starts_with(['^', 'v', '<', '>']))
        .flat_map(|steps| steps.chars());

    // part 1
    let mut robot: Point = ROBOT;
    for step in steps.clone() {
        // there will ALWAYS be wall
        let wall = robot.get_obj('#', None, step, &map);

        if let Some(free_space) = robot.get_obj('.', wall, step, &map) {
            // free space before wall, take a step
            if let Some(carton) = robot.get_obj('O', Some(free_space), step, &map) {
                // at least one carton before free space
                map[free_space.x][free_space.y] = 'O';
                map[carton.x][carton.y] = '@';
                map[robot.x][robot.y] = '.';
                robot = carton;
            } else {
                // no cartons before free space
                map[free_space.x][free_space.y] = '@';
                map[robot.x][robot.y] = '.';
                robot = free_space;
            }
        }
        // otherwise skip taking this step/move
    }

    let mut sum = 0usize;
    (1..EDGE - 1).for_each(|y| {
        (1..EDGE - 1).for_each(|x| {
            if map[x][y] == 'O' {
                sum += x + 100 * y;
            }
        })
    });

    println!("part1: {sum}");

    let mut robot: Point = Point::new(ROBOT.x * 2, ROBOT.y);
    let mut shift_coords: VecDeque<(Point, Point)>;

    for step in steps {
        shift_coords = robot.take_step(step, &wide_map);

        while let Some((from, to)) = shift_coords.pop_back() {
            wide_map[to.x][to.y] = wide_map[from.x][from.y];
            wide_map[from.x][from.y] = '.';
            robot = to;
        }
    }

    let mut sum = 0usize;

    (1..EDGE - 1).for_each(|y| {
        (2..2 * (EDGE - 1)).for_each(|x| {
            if wide_map[x][y] == '[' {
                sum += x + 100 * y;
            }
        })
    });

    println!("part2: {sum}");
}
