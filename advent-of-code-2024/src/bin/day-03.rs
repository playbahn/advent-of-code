fn mul(input_slice: &str) -> u32 {
    let comma = if let Some(comma) = input_slice[4..8].find(",") {
        4 + comma
    } else {
        return 0;
    };

    let ins_end = if let Some(ins_end) = input_slice[comma + 2..comma + 5].find(")") {
        comma + 2 + ins_end
    } else {
        return 0;
    };

    if let (Ok(x), Ok(y)) = (
        input_slice[4..comma].parse::<u32>(),
        input_slice[1 + comma..ins_end].parse::<u32>(),
    ) {
        x * y
    } else {
        0
    }
}

fn sum_for_substring(input: &str) -> u64 {
    input
        .match_indices("mul(")
        .map(|(idx, _)| idx)
        .fold(0, |sum, idx| sum + mul(&input[idx..idx + 12]) as u64)
}

fn main() {
    let input: String = std::fs::read_to_string("input/day-03.txt").unwrap();

    let flag_start = input
        .find("do()")
        .unwrap()
        .min(input.find("don't()").unwrap());
    
    let common = sum_for_substring(&input[..flag_start]);

    // part 1
    let res_1 = common + sum_for_substring(&input[4 + flag_start..]);
    println!("res_1: {res_1}");

    // part 2
    let mut vec = input.match_indices("do()").collect::<Vec<_>>();
    vec.extend(input.match_indices("don't()"));
    vec.sort_by(|flag1, flag2| flag1.0.cmp(&flag2.0));
    vec.dedup_by(|flag1, flag2| flag1.1 == flag2.1);

    let res_2 = common
        + vec.windows(2).fold(0, |sum, flags| {
            sum + if flags[0].1 == "do()" {
                sum_for_substring(&input[flags[0].0 + 4..flags[1].0])
            } else {
                0
            }
        });

    println!("res_2: {res_2}");
}
