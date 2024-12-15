const INPUT: &str = "input/day-15.txt";
const EDGE: usize = 50;

#[derive(Clone, Copy)]
struct Coords {
    x: usize,
    y: usize,
}

impl Coords {
    const LB: Self = Self { x: 0, y: 0 };
    const UB: Self = Self { x: EDGE, y: EDGE };

    /// Returns `Some(coords)` if `object` comes before `bounds`, otherwise returns
    /// `None`. If `bounds` is `None`, `Coords::UB` and `Coords::LB` are used
    /// instead, depending on `step`.
    ///
    /// # Panics
    ///
    /// Panics if `step` isn't any one of `'^'`, `'v'`, `'<'`, `'>'`.
    fn obj_before_bounds(
        &self,
        object: char,
        bounds: Option<Self>,
        step: char,
        depot: &[[char; EDGE]; EDGE],
    ) -> Option<Self> {
        match step {
            '^' => (bounds.unwrap_or(Self::LB).y..self.y)
                .filter_map(|y| (depot[self.x][y] == object).then_some(Self { x: self.x, y }))
                .max_by(|coords1, coords2| coords1.y.cmp(&coords2.y)),
            'v' => (self.y + 1..bounds.unwrap_or(Self::UB).y)
                .filter_map(|y| (depot[self.x][y] == object).then_some(Self { x: self.x, y }))
                .min_by(|coords1, coords2| coords1.y.cmp(&coords2.y)),
            '<' => (bounds.unwrap_or(Self::LB).x..self.x)
                .filter_map(|x| (depot[x][self.y] == object).then_some(Self { x, y: self.y }))
                .max_by(|coords1, coords2| coords1.x.cmp(&coords2.x)),
            '>' => (self.x + 1..bounds.unwrap_or(Self::UB).x)
                .filter_map(|x| (depot[x][self.y] == object).then_some(Self { x, y: self.y }))
                .min_by(|coords1, coords2| coords1.x.cmp(&coords2.x)),
            _ => panic!(),
        }
    }
}

fn main() {
    let input = std::fs::read_to_string(INPUT).unwrap();

    let mut depot: [[char; EDGE]; EDGE] = [['\0'; EDGE]; EDGE];
    input
        .lines()
        .take_while(|line| !line.is_empty())
        .enumerate()
        .for_each(|(y, line)| {
            line.char_indices()
                .for_each(|(x, object)| depot[x][y] = object)
        });

    let mut robot: Coords = Coords { x: 24, y: 24 };

    let steps = input
        .lines()
        .skip_while(|line| !line.starts_with(['^', 'v', '<', '>']))
        .flat_map(|moves| moves.chars());

    for step in steps {
        let wall = robot.obj_before_bounds('#', None, step, &depot);

        if let Some(free_space) = robot.obj_before_bounds('.', wall, step, &depot) {
            if let Some(carton) = robot.obj_before_bounds('O', Some(free_space), step, &depot) {
                depot[free_space.x][free_space.y] = 'O';
                depot[carton.x][carton.y] = '@';
                depot[robot.x][robot.y] = '.';
                robot = carton;
            } else {
                depot[free_space.x][free_space.y] = '@';
                depot[robot.x][robot.y] = '.';
                robot = free_space;
            }
        }
    }

    let mut sum = 0usize;

    (1..EDGE - 1).for_each(|y| {
        (1..EDGE - 1).for_each(|x| {
            if depot[x][y] == 'O' {
                sum += x + 100 * y;
            }
        })
    });

    println!("part1: {sum}");

    let mut wide_depot: [[char; EDGE]; EDGE << 1] = [['\0'; EDGE]; EDGE << 1];

    (0..EDGE).for_each(|y| {
        (0..EDGE).for_each(|x| {
            (wide_depot[x << 1][y], wide_depot[(x << 1) + 1][y]) = if depot[x][y] == 'O' {
                ('[', ']')
            } else {
                (depot[x][y], depot[x][y])
            }
        })
    });

    wide_depot[49][24] = '.';

    (0..EDGE).for_each(|y| {
        (0..EDGE << 1).for_each(|x| print!("{}", wide_depot[x][y]));
        println!();
    });

    let mut robot: Coords = Coords { x: 48, y: 24 };
}
