use std::collections::HashSet;

const PATH: &str = "input/day-10.txt";

type Coords = (usize, usize);

fn get_peaks(curr: Coords, map: &[[u8; 59]; 59], peaks: &mut HashSet<Coords>, rating: &mut u16) {
    if map[curr.0][curr.1] == 9 {
        peaks.insert(curr);
        *rating += 1;
        return;
    }

    if let Some(x) = curr.0.checked_sub(1) {
        if map[x][curr.1] == 1 + map[curr.0][curr.1] {
            get_peaks((x, curr.1), map, peaks, rating);
        }
    }

    if 1 + curr.0 < 59 && map[1 + curr.0][curr.1] == 1 + map[curr.0][curr.1] {
        get_peaks((1 + curr.0, curr.1), map, peaks, rating);
    }

    if let Some(y) = curr.1.checked_sub(1) {
        if map[curr.0][y] == 1 + map[curr.0][curr.1] {
            get_peaks((curr.0, y), map, peaks, rating);
        }
    }

    if 1 + curr.1 < 59 && map[curr.0][1 + curr.1] == 1 + map[curr.0][curr.1] {
        get_peaks((curr.0, 1 + curr.1), map, peaks, rating);
    }
}

fn main() {
    let mut map: [[u8; 59]; 59] = [[255; 59]; 59];
    let mut heads: Vec<Coords> = Vec::new();

    std::fs::read_to_string(PATH)
        .unwrap()
        .lines()
        .enumerate()
        .for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, ch)| {
                map[x][y] = ch.to_digit(10).unwrap() as u8;
                if map[x][y] == 0 {
                    heads.push((x, y))
                }
            })
        });

    let mut part1 = 0usize;
    let mut part2 = 0u16;
    let mut peaks: HashSet<Coords> = HashSet::new();

    for head in heads {
        get_peaks(head, &map, &mut peaks, &mut part2);
        part1 += peaks.len();
        peaks.clear();
    }

    println!("part1: {part1} part2: {part2}")
}
