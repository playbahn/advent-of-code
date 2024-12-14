const INPUT: &str = "input/day-14.txt";

const UBX: u8 = 101;
const UBY: u8 = 103;
const TIME: u8 = 100;

/// Position
struct P {
    x: u8,
    y: u8,
}

/// Velocity
type V = P;
/// Movement
type M = P;

impl std::ops::Add<M> for P {
    type Output = P;

    fn add(self, rhs: M) -> Self::Output {
        P {
            x: (self.x + rhs.x) % UBX,
            y: (self.y + rhs.y) % UBY,
        }
    }
}

/// Part 2 straight up mocked this :_)
impl std::ops::Mul<u8> for V {
    type Output = M;

    fn mul(self, rhs: u8) -> Self::Output {
        M {
            x: ((self.x as u16 * rhs as u16) % UBX as u16) as u8,
            y: ((self.y as u16 * rhs as u16) % UBY as u16) as u8,
        }
    }
}

fn pvs(input: &str) -> impl Iterator<Item = (P, V)> + '_ {
    input
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
                P {
                    x: p.0.parse().unwrap(),
                    y: p.1.parse().unwrap(),
                },
                V {
                    x: (v.0.parse::<i16>().unwrap() + UBX as i16) as u8 % UBX,
                    y: (v.1.parse::<i16>().unwrap() + UBY as i16) as u8 % UBY,
                },
            )
        })
}

fn get_quadrant(p: P) -> Option<usize> {
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
    let mut quadrants: [u16; 4] = [0, 0, 0, 0];

    for (p, v) in pvs(&std::fs::read_to_string(INPUT).unwrap()) {
        if let Some(quadrant) = get_quadrant(p + v * TIME) {
            quadrants[quadrant] += 1;
        }
    }

    let safety_factor: u32 = quadrants.iter().map(|num| *num as u32).product();
    println!("Part 1: {}", safety_factor);
    
    
}
