use std::collections::HashMap;

fn main() {
    // common
    let (mut left_list, mut right_list): (Vec<u32>, Vec<u32>) =
        std::fs::read_to_string("input/day-01.txt")
            .unwrap()
            .lines()
            .map(|line| line.split_whitespace().collect::<Vec<&str>>())
            .fold(
                (Vec::with_capacity(1000), Vec::with_capacity(1000)),
                |(mut list1, mut list2), pair_vec| {
                    list1.push(pair_vec[0].parse().unwrap());
                    list2.push(pair_vec[1].parse().unwrap());
                    (list1, list2)
                },
            );

    left_list.sort();
    right_list.sort();

    // part 1
    let mut dist = 0;

    for (left_id, right_id) in left_list.iter().zip(right_list.iter()) {
        dist += left_id.abs_diff(*right_id);
    }

    println!("dist: {dist}");

    // part 2
    let mut right_id_freq: HashMap<u32, usize> = HashMap::with_capacity(1000);
    let mut similarity_score = 0;

    // all right ids and how many times they appear
    for right_id in right_list {
        right_id_freq
            .entry(right_id)
            .and_modify(|freq| *freq += 1)
            .or_insert(1);
    }

    for left_id in left_list {
        similarity_score += right_id_freq.get(&left_id).unwrap_or(&0) * left_id as usize;
    }

    println!("similarity_score: {similarity_score}");
}
