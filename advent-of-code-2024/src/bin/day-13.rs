const INPUT: &str = "input/day-13.txt";
const CORRECTION: u64 = 10_000_000_000_000;

fn main() {
    let mut consts = [[0f64; 2]; 3];
    let (mut a_pressed, mut b_pressed);
    let mut tokens_1 = 0u16;
    let mut tokens_2 = 0u64;

    for machine in std::fs::read_to_string(INPUT)
        .unwrap()
        .split_terminator("\n\n")
    {
        machine.lines().enumerate().for_each(|(line, constants)| {
            constants[8..][if line == 2 { 0.. } else { 3.. }]
                // leaves `+` or `=` before each number
                .split(", Y")
                .enumerate()
                .for_each(|(axis, constant)| {
                    // parse from after `+` or `=`
                    consts[line][axis] = constant[1..].parse().unwrap()
                })
        });

        // infinite solutions: a1/a2 == b1/b2 == c1/c2
        // no solutions: a1/a2 == b1/b2 != c1/c2
        // unique solution: a1/a2 != b1/b2
        // NOT required to check any, print debugging
        // shows all cases have unique solutions
        // if consts[0][0] / consts[0][1] != consts[1][0] / consts[1][1] {
        //     println!("unique");
        // } else if consts[1][0] / consts[1][1] != consts[2][0] / consts[2][1] {
        //     println!("none: pt1");
        // } else if consts[1][0] / consts[1][1]
        //     != (consts[2][0] + CORRECTION as f64) / (consts[2][1] + CORRECTION as f64)
        // {
        //     println!("none: pt2");
        // } else {
        //     println!("infinite");
        // }

        // `consts`:
        // a1: [0][0], b1: [1][0], c1: [2][0]
        // a2: [0][1], b2: [1][1], c2: [2][1]
        // For the first example machine
        // Button A: X+94, Y+34
        // Button B: X+22, Y+67
        // Prize: X=8400, Y=5400
        // 94 * `a_pressed` + 22 * `b_pressed` = 8400

        // part 1
        // a_pressed = (b1*c2 - b2*c1)/(a2*b1 - a1*b2)
        a_pressed = (consts[1][0] * consts[2][1] - consts[1][1] * consts[2][0])
            / (consts[0][1] * consts[1][0] - consts[0][0] * consts[1][1]);
        // b_pressed = (c1 - a1*`a_pressed`)/b1
        b_pressed = (consts[2][0] - consts[0][0] * a_pressed) / consts[1][0];

        // from https://adventofcode.com/2024/day/13:
        // > each button would need to be pressed no more than 100 times.
        // Decimal places in `< 100.1` do not matter, we check beforehand
        // that `.fract() == 0.0`, `< 100.1` here is just another way of
        // writing `<=100.0`
        if a_pressed.fract() == 0.0
            && b_pressed.fract() == 0.0
            && a_pressed < 100.1
            && b_pressed < 100.1
        {
            tokens_1 += 3 * a_pressed as u16 + b_pressed as u16;
        }

        // part 2
        // a_pressed = {b1*(c2 + CORRECTION) - b2*(c1 + CORRECTION)}/(a2*b1 - a1*b2)
        a_pressed = (consts[1][0] * (consts[2][1] + CORRECTION as f64)
            - consts[1][1] * (consts[2][0] + CORRECTION as f64))
            / (consts[0][1] * consts[1][0] - consts[0][0] * consts[1][1]);
        // b_pressed = {(c1 + CORRECTION) - a1*`a_pressed`}/b1
        b_pressed = ((consts[2][0] + CORRECTION as f64) - consts[0][0] * a_pressed) / consts[1][0];

        // from https://adventofcode.com/2024/day/13#part2:
        // > Unfortunately, it will take many more than 100 presses to do so.
        // No more `< 100.1` checks
        if a_pressed.fract() == 0.0 && b_pressed.fract() == 0.0 {
            tokens_2 += 3 * a_pressed as u64 + b_pressed as u64;
        }
    }

    println!("tokens_1: {tokens_1} tokens_2: {tokens_2}");
}
