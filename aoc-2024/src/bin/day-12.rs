use std::collections::{HashMap, HashSet};

const INPUT: &str = "input/day-12.txt";
const UB: usize = 140;

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

struct Region {
    x: HashMap<usize, Vec<usize>>,
    y: HashMap<usize, Vec<usize>>,
}

impl Region {
    fn insert(&mut self, plot: Point) {
        self.x
            .entry(plot.x)
            .and_modify(|ys| ys.push(plot.y))
            .or_insert(vec![plot.y]);

        self.y
            .entry(plot.y)
            .and_modify(|xs| xs.push(plot.x))
            .or_insert(vec![plot.x]);
    }

    fn clear(&mut self) {
        self.x.clear();
        self.y.clear();
    }
}

fn get_region(
    plot: Point,
    region: &mut Region,
    unvisited: &mut HashSet<Point>,
    garden: &[[char; UB]; UB],
) {
    unvisited.remove(&plot);
    region.insert(plot);

    let plant = garden[plot.x][plot.y];
    let mut next_plots = Vec::with_capacity(4);

    if plot.x > 0 && garden[plot.x - 1][plot.y] == plant {
        next_plots.push(Point::new(plot.x - 1, plot.y));
    }

    if plot.x + 1 < UB && garden[plot.x + 1][plot.y] == plant {
        next_plots.push(Point::new(plot.x + 1, plot.y));
    }

    if plot.y > 0 && garden[plot.x][plot.y - 1] == plant {
        next_plots.push(Point::new(plot.x, plot.y - 1));
    }

    if plot.y + 1 < UB && garden[plot.x][plot.y + 1] == plant {
        next_plots.push(Point::new(plot.x, plot.y + 1));
    }

    next_plots.into_iter().for_each(|next_plot| {
        if unvisited.contains(&next_plot) {
            get_region(next_plot, region, unvisited, garden);
        }
    })
}

fn costs(region: &Region, garden: &[[char; UB]; UB]) -> (usize, usize) {
    let mut p = 0;
    let mut s = 0;

    region.x.iter().for_each(|(x, ys)| {
        let mut edges_l: Vec<&usize> = ys
            .iter()
            .filter(|&&y| *x == 0 || garden[x - 1][y] != garden[*x][y])
            .collect();

        let mut edges_r: Vec<&usize> = ys
            .iter()
            .filter(|&&y| *x + 1 == UB || garden[x + 1][y] != garden[*x][y])
            .collect();

        p += edges_l.len() + edges_r.len();

        edges_l.sort();
        edges_r.sort();

        s += if edges_l.is_empty() { 0 } else { 1 }
            + edges_l.windows(2).filter(|ys| ys[0] + 1 != *ys[1]).count()
            + if edges_r.is_empty() { 0 } else { 1 }
            + edges_r.windows(2).filter(|ys| ys[0] + 1 != *ys[1]).count();
    });

    region.y.iter().for_each(|(y, xs)| {
        let mut edges_t: Vec<&usize> = xs
            .iter()
            .filter(|&&x| *y == 0 || garden[x][y - 1] != garden[x][*y])
            .collect();

        let mut edges_b: Vec<&usize> = xs
            .iter()
            .filter(|&&x| *y + 1 == UB || garden[x][*y + 1] != garden[x][*y])
            .collect();

        p += edges_t.len() + edges_b.len();

        edges_t.sort();
        edges_b.sort();

        s += if edges_t.is_empty() { 0 } else { 1 }
            + edges_t.windows(2).filter(|xs| xs[0] + 1 != *xs[1]).count()
            + if edges_b.is_empty() { 0 } else { 1 }
            + edges_b.windows(2).filter(|xs| xs[0] + 1 != *xs[1]).count();
    });

    let a = region.x.iter().fold(0, |a, (_, ys)| a + ys.len());

    (a * p, a * s)
}

fn main() {
    let start = std::time::Instant::now();
    let mut garden: [[char; UB]; UB] = [['\0'; UB]; UB];
    // to keep track of unvisited plots; `V` = `HashSet<_>` just for easy deletion.
    let mut unvisited: HashMap<char, HashSet<Point>> =
        HashMap::from_iter(('A'..'[').map(|ch| (ch, HashSet::new())));

    std::fs::read_to_string(INPUT)
        .unwrap()
        .lines()
        .enumerate()
        .for_each(|(y, isle)| {
            isle.char_indices().for_each(|(x, plant)| {
                garden[x][y] = plant;
                unvisited.get_mut(&plant).unwrap().insert(Point::new(x, y));
            })
        });

    let mut part1 = 0;
    let mut part2 = 0;

    let mut start_plot = Point::new(usize::MAX, usize::MAX);
    let mut plant = '\0';
    let (mut c1, mut c2) = (0, 0);
    let mut region = Region {
        x: HashMap::new(),
        y: HashMap::new(),
    };

    (0..UB).for_each(|y| {
        (0..UB).for_each(|x| {
            start_plot = Point::new(x, y);
            plant = garden[x][y];

            if unvisited[&plant].contains(&start_plot) {
                get_region(
                    start_plot,
                    &mut region,
                    unvisited.get_mut(&plant).unwrap(),
                    &garden,
                );

                (c1, c2) = costs(&region, &garden);
                part1 += c1;
                part2 += c2;
                region.clear();
            }
        });
    });

    println!("{}s", start.elapsed().as_secs_f64());
    println!("part1: {part1}\npart2: {part2}");
}

// Was used for part 1
// struct _Region {
//     a: u32,
//     p: u32,
// }

// impl std::ops::AddAssign for _Region {
//     fn add_assign(&mut self, rhs: Self) {
//         self.a += rhs.a;
//         self.p += rhs.p;
//     }
// }

// fn _region_ap(
//     start: Point,
//     unvisited: &mut HashSet<Point>,
//     garden: &[[char; UB]; UB],
//     plant: char,
// ) -> _Region {
//     let mut region = _Region { a: 1, p: 4 };

//     unvisited.remove(&start);

//     if start.0 > 0 && garden[start.0 - 1][start.1] == plant {
//         region.p -= 1;
//         if unvisited.contains(&(start.0 - 1, start.1)) {
//             region += _region_ap((start.0 - 1, start.1), unvisited, garden, plant);
//         }
//     }

//     if start.0 + 1 < UB && garden[start.0 + 1][start.1] == plant {
//         region.p -= 1;
//         if unvisited.contains(&(start.0 + 1, start.1)) {
//             region += _region_ap((start.0 + 1, start.1), unvisited, garden, plant);
//         }
//     }

//     if start.1 > 0 && garden[start.0][start.1 - 1] == plant {
//         region.p -= 1;
//         if unvisited.contains(&(start.0, start.1 - 1)) {
//             region += _region_ap((start.0, start.1 - 1), unvisited, garden, plant);
//         }
//     }

//     if start.1 + 1 < UB && garden[start.0][start.1 + 1] == plant {
//         region.p -= 1;
//         if unvisited.contains(&(start.0, start.1 + 1)) {
//             region += _region_ap((start.0, start.1 + 1), unvisited, garden, plant);
//         }
//     }

//     region
// }
