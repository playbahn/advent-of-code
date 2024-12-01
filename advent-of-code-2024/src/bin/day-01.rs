use std::collections::HashMap;

fn main() {
    // common
    let (mut list1, mut list2): (Vec<u32>, Vec<u32>) = std::fs::read_to_string("input/day-01.txt")
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

    list1.sort();
    list2.sort();

    // part 1
    let mut dist = 0;

    for (num1, num2) in list1.iter().zip(list2.iter()) {
        dist += num1.abs_diff(*num2);
    }

    println!("dist: {dist}");

    // part 2
    let mut right_num_freq: HashMap<u32, usize> = HashMap::with_capacity(1000);
    let mut similarity_score = 0;

    // all right nums and how many times they appear
    for right_num in list2 {
        right_num_freq
            .entry(right_num)
            .and_modify(|freq| *freq += 1)
            .or_insert(1);
    }

    for left_num in list1 {
        similarity_score += left_num as usize * right_num_freq.get(&left_num).unwrap_or(&0);
    }

    println!("similarity_score: {similarity_score}");
}
