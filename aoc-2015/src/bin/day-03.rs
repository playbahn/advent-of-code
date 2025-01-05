use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("input/day-03.txt").unwrap();
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert((0, 0));

    // PART1
    input
        .chars()
        .fold((0, 0), |curr_coords: (i32, i32), direction| {
            let new_coords: (i32, i32) = match direction {
                '>' => (curr_coords.0 + 1, curr_coords.1),
                '<' => (curr_coords.0 - 1, curr_coords.1),
                '^' => (curr_coords.0, curr_coords.1 + 1),
                'v' => (curr_coords.0, curr_coords.1 - 1),
                _ => panic!(),
            };
            visited.insert(new_coords);
            new_coords
        });

    println!("{}", visited.len());

    // PART2 - Santa
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert((0, 0));

    input
        .chars()
        .step_by(2)
        .fold((0, 0), |curr_coords: (i32, i32), direction| {
            let new_coords: (i32, i32) = match direction {
                '>' => (curr_coords.0 + 1, curr_coords.1),
                '<' => (curr_coords.0 - 1, curr_coords.1),
                '^' => (curr_coords.0, curr_coords.1 + 1),
                'v' => (curr_coords.0, curr_coords.1 - 1),
                _ => panic!(),
            };
            visited.insert(new_coords);
            new_coords
        });

    // PART2 - Robo
    input
        .chars()
        .skip(1)
        .step_by(2)
        .fold((0, 0), |curr_coords: (i32, i32), direction| {
            let new_coords: (i32, i32) = match direction {
                '>' => (curr_coords.0 + 1, curr_coords.1),
                '<' => (curr_coords.0 - 1, curr_coords.1),
                '^' => (curr_coords.0, curr_coords.1 + 1),
                'v' => (curr_coords.0, curr_coords.1 - 1),
                _ => panic!(),
            };
            visited.insert(new_coords);
            new_coords
        });

    println!("{}", visited.len());
}
