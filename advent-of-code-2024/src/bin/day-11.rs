const PATH: &str = "input/day-11.txt";

#[memoize::memoize]
fn count_stones(stones: Vec<String>, blinks: u8) -> usize {
    if blinks == 0 {
        return stones.len();
    }

    let mut count = 0usize;
    
    stones.iter().for_each(|stone| {
        let new_stones: Vec<String> = if stone == "0" {
            vec!["1".to_string()]
        } else if stone.len() % 2 == 0 {
            vec![
                stone[..stone.len() / 2].to_string(),
                stone[stone.len() / 2..].parse::<u32>().unwrap().to_string(),
            ]
        } else {
            vec![(stone.parse::<u64>().unwrap() * 2024).to_string()]
        };
        count += count_stones(new_stones, blinks - 1);
    });

    count
}

fn main() {
    let stones: Vec<String> = Vec::from_iter(
        std::fs::read_to_string(PATH)
            .unwrap()
            .split_whitespace()
            .map(|num| num.to_owned()),
    );

    let part1 = count_stones(stones.clone(), 25);
    println!("stones after 25 blinks: {part1}");

    let part2 = count_stones(stones, 75);
    println!("stones after 75 blinks: {part2}");
}
