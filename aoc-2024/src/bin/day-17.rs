const INPUT: &str = "input/day-17.txt";

// *** NOTE ***
// Solution fine-grained for my specific input,
// WILL NOT work for anybody else for which the effective
// program is different than the `_program: loop`
fn program(mut a: u64) -> Vec<u8> {
    let (mut b, mut c);
    let mut out: Vec<u8> = vec![];

    '_program: loop {
        b = (a as u8 & 0b111) ^ 0b010;
        c = (a >> b) as u8 & 0b111;
        out.push(b ^ c ^ 0b011);
        a >>= 3;
        if a == 0 {
            break;
        }
    }

    out
}

fn append_a(prog: &Vec<u8>, index: usize, a: u64) -> Option<u64> {
    // NOTE: We "build up" register A starting from the MSB of
    // "more-than-45-less-than-49-bits-lengthed" register A
    // for any `x`, `y` and `z`, `x^y = z` => `x^z = y` => `y^z = x`
    // for any `prog[index]` printed,
    // `b^c^0b011 = prog[index]` => `b^c = 0b011^prog[index]`
    let b_xor_c: u8 = 0b011 ^ prog[index];
    // all candidate pairs (`b`,`c`) such that `b`^`c` = `0b011^prog[index]`
    for (b, c) in (0b000..0b111 + 1).map(|b| (b, b ^ b_xor_c)) {
        // from `program`:
        // b = (a as u8 & 0b111) ^ 0b010;
        // c = (a >> b) as u8 & 0b111;
        // for any candidates `b` and `c` in this `for loop`,
        // b = (a as u8 & 0b111) ^ 0b010 =>
        // (last-3-bits-of-A-before-its-shifted-down) ^ 0b010 = b
        // We substitute b to get:
        // (last-3-bits-of-A-before-its-shifted-down) = `b`^0b010 = append
        let append = 0b010 ^ b;
        // We append (verb, not variable) this `append` (variable, not verb)
        // to the end of A, let's call this `new_a` (still not sure about it
        // being the "actual" new A)
        let new_a = (a << 3) | append as u64;
        // We shift down `new_a` by `b` bits, take the last 3 bits,
        // let's call this `check_c`
        let check_c = (new_a >> b) as u8 & 0b111;
        if c == check_c {
            // So, c = check_c, maybe we're on right path
            if index > 0 {
                // index > 0, means still output numbers left to calculate
                // for in `prog`
                if let Some(new_a) = append_a(prog, index - 1, new_a) {
                    // if `append_a` returns `Some(A)` we return it,
                    // otherwise, `None` means, although check_c = c,
                    // this (b,c) results in some unsuitable intermediate
                    // value for A, for which, for the next output
                    // numbers, no (b,c) are found. We continue to check
                    // for other (b,c) in this for loop.
                    return Some(new_a);
                }
            } else {
                // check_c == c && index == 0 => there's no more numbers
                // left in program, the right (b,c) in this exact recursive
                // root to leaf recursive branch.
                return Some(new_a);
            }
        }
    }

    // No (b,c) found, somewhere up the recursion tree,
    // an unsuitable (b,c) was chosen
    None
}

fn main() {
    let a: u64 = std::fs::read_to_string(INPUT).unwrap()[12..20]
        .parse()
        .unwrap();

    let out = program(a);
    println!("part 1: {out:?}");

    let prog: Vec<u8> = std::fs::read_to_string(INPUT).unwrap().trim_end()[59..]
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    // At program halt, A = 0, so we use it as the starting build up value for A
    let quine_a: u64 = append_a(&prog, prog.len() - 1, 0).expect("NO suitable A found.");

    println!("quine_a: {quine_a} (0b{quine_a:b})");
}
