/// NOT TO SELF - DO NOT EVER CREATE/USE/IMPLEMENT
/// CRATE TYPES (e.g. STRUCTS) FOR PROBLEM SOLVING
/// EVER AGAIN - OR MAYBE DO I GUESS
use std::collections::HashSet;

const INPUT: &str = "input/day-14.txt";

const UBX: u8 = 101;
const UBY: u8 = 103;
const TIME: u16 = 100;
const LCM_UBX_UBY: u16 = UBX as u16 * UBY as u16;

#[derive(PartialEq, Eq, Hash, Clone)]
struct Position {
    x: u8,
    y: u8,
}

type Displacement = Position;

impl std::ops::Mul<u16> for &Displacement {
    type Output = Displacement;

    fn mul(self, rhs: u16) -> Self::Output {
        Displacement {
            x: ((self.x as u16 * rhs) % UBX as u16) as u8,
            y: ((self.y as u16 * rhs) % UBY as u16) as u8,
        }
    }
}

impl std::ops::Add<Displacement> for &Position {
    type Output = Position;

    fn add(self, rhs: Displacement) -> Self::Output {
        Position {
            x: (self.x + rhs.x) % UBX,
            y: (self.y + rhs.y) % UBY,
        }
    }
}

impl std::ops::AddAssign<&Displacement> for Position {
    fn add_assign(&mut self, rhs: &Displacement) {
        self.x = (self.x + rhs.x) % UBX;
        self.y = (self.y + rhs.y) % UBY;
    }
}

fn get_quadrant(p: Position) -> Option<usize> {
    use std::cmp::Ordering::*;

    match (p.x.cmp(&(UBX >> 1)), p.y.cmp(&(UBY >> 1))) {
        (Less, Less) => Some(0),
        (Greater, Less) => Some(1),
        (Less, Greater) => Some(2),
        (Greater, Greater) => Some(3),
        _ => None,
    }
}

fn main() {
    let (positions, displacements): (Vec<Position>, Vec<Displacement>) =
        std::fs::read_to_string(INPUT)
            .unwrap()
            .lines()
            .map(|robot| robot.split_once(' ').unwrap())
            .map(|(p, v)| {
                (
                    p[2..].split_once(',').unwrap(),
                    v[2..].split_once(',').unwrap(),
                )
            })
            .map(|(p, v)| {
                (
                    Position {
                        x: p.0.parse().unwrap(),
                        y: p.1.parse().unwrap(),
                    },
                    Displacement {
                        x: (v.0.parse::<i16>().unwrap() + UBX as i16) as u8 % UBX,
                        y: (v.1.parse::<i16>().unwrap() + UBY as i16) as u8 % UBY,
                    },
                )
            })
            .collect();

    let mut quadrants: [u16; 4] = [0, 0, 0, 0];
    for (position, displacement) in positions.iter().zip(displacements.iter()) {
        if let Some(quadrant) = get_quadrant(position + displacement * TIME) {
            quadrants[quadrant] += 1;
        }
    }

    let safety_factor: u32 = quadrants.iter().map(|num| *num as u32).product();
    println!("Part 1: {}", safety_factor);

    let mut positions: Vec<Position> = positions;
    let mut distincts: HashSet<Position> = HashSet::with_capacity(500);
    let mut set_pixels: Vec<Position> = Vec::with_capacity(500);
    let mut img = bmp::Image::new(UBX as u32, UBY as u32);

    println!("Writing bitmaps to img/ ...");
    // Every x-coord will be at its original position after UBX time-steps, and y-coord
    // after UBY. Both of the x- and y-corods will be at their original position
    // simultaneously at LCM(UBX, UBY) = `LCM_UBX_UBY` time-steps. After which, the
    // same coords will start repeating.
    for time in 1..LCM_UBX_UBY {
        for (position, displacement) in positions.iter_mut().zip(displacements.iter()) {
            *position += displacement;
        }

        distincts.extend(positions.clone());
        for y in 0..UBY {
            for x in 0..UBX {
                if distincts.contains(&Position { x, y }) {
                    img.set_pixel(x as u32, y as u32, bmp::consts::RED);
                    set_pixels.push(Position { x, y });
                }
            }
        }
        
        img.save(format!("img/{}.bmp", time)).unwrap();
        for set_pixel in &set_pixels {
            img.set_pixel(set_pixel.x as u32, set_pixel.y as u32, bmp::consts::BLACK);
        }
        
        distincts.clear();
        set_pixels.clear();
    }
    println!();
}
