fn combo(operand: u8, a: u128, b: u128, c: u128) -> u128 {
    match operand {
        4 => a,
        5 => b,
        6 => c,
        // 7 will not appear as combo operand in valid
        // programs, no need to check
        _ => operand as u128,
    }
}

fn div(a: u128, combo: u128) -> u128 {
    if let Some(denominator) = 2u128.checked_pow(combo as u32) {
        a / denominator
    } else {
        0
    }
}

fn main() {
    let input: Vec<String> = std::fs::read_to_string("input/day-17.txt")
        .unwrap()
        .lines()
        .map(|line| line.to_string())
        .collect();

    let (mut a, mut b, mut c): (u128, u128, u128) = (
        input[0][12..].parse().unwrap(),
        input[1][12..].parse().unwrap(),
        input[2][12..].parse().unwrap(),
    );

    let program: Vec<u8> = input[4][9..]
        .split(',')
        .map(|num| num.parse().unwrap())
        .collect();

    let mut out: Vec<u128> = vec![];
    let mut jump;
    let mut ptr = 0usize;

    // dbg!(ptr, a, b, c);

    while ptr < program.len() {
        jump = false;
        // println!("{}", program[ptr]);
        match program[ptr] {
            0 => a = div(a, combo(program[ptr + 1], a, b, c)),
            1 => b ^= program[ptr + 1] as u128,
            2 => b = combo(program[ptr + 1], a, b, c) % 8,
            3 => {
                if a > 0 {
                    ptr = program[ptr + 1] as usize;
                    // println!("jump to {ptr}");
                    jump = true;
                }
            }
            4 => b ^= c,
            5 => out.push(combo(program[ptr + 1], a, b, c) % 8),
            7 => c = div(a, combo(program[ptr + 1], a, b, c)),
            _ => panic!(),
        }

        if !jump {
            ptr += 2;
        }
        
        // dbg!(ptr, a, b, c);
    }

    println!("{out:?}");
}
