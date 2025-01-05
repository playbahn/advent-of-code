use dinglebit_combinatorics::*;

fn main() {
    let containers: [u8; 20] = [
        1, 1, 3, 3, 3, 5, 11, 11, 15, 19, 26, 28, 30, 31, 32, 32, 36, 36, 46, 47,
    ];
    let mut combinations: Vec<Vec<usize>> = Vec::new();
    let mut suitable_combinations: usize = 0;

    for k in 4..13 {
        for c in Combination::new(20, k) {
            combinations.push(c);
        }
    }

    // println!("{}", combinations.len());

    let mut min_containers: usize = std::usize::MAX;
    let mut min_ways: usize = 0;
    let mut sum: u8;
    'combination: for combination in &combinations {
        sum = 0;
        for idx in combination {
            sum += containers[*idx];
            if sum > 150 {
                continue 'combination;
            }
        }
        if sum == 150 {
            suitable_combinations += 1;
            if min_containers > combination.len() {
                min_containers = combination.len();
                min_ways = 1;
            } else if min_containers == combination.len() {
                min_ways += 1;
            }
        }
    }

    println!("{}", suitable_combinations);
    println!("{}", min_ways);
}
