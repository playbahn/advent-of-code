enum Part {
    One,
    Two,
}

fn prep_op_seqs(count: usize, part: &Part) -> Vec<Vec<char>> {
    let mut seqs: Vec<Vec<char>> = vec![];
    op_seqs_aux(&mut seqs, vec![], count, part);
    seqs
}

fn op_seqs_aux(seqs: &mut Vec<Vec<char>>, mut seq_a: Vec<char>, count: usize, part: &Part) {
    if count == 0 {
        seqs.push(seq_a);
    } else {
        if let Part::Two = part {
            let mut seq_c = seq_a.clone();
            seq_c.push('|');
            op_seqs_aux(seqs, seq_c, count - 1, part);
        }
        let mut seq_b = seq_a.clone();
        seq_a.push('+');
        seq_b.push('*');
        op_seqs_aux(seqs, seq_a, count - 1, part);
        op_seqs_aux(seqs, seq_b, count - 1, part);
    }
}

fn main() {
    let eqs: Vec<Vec<u64>> = Vec::from_iter(
        std::fs::read_to_string("input/day-07.txt")
            .unwrap()
            .lines()
            .map(|eq| Vec::from_iter(eq.split_whitespace().map(|eq_val| eq_val.parse().unwrap()))),
    );

    let mut part1 = 0u64;
    let mut part2 = 0u64;

    for eq in eqs {
        for op_seq in prep_op_seqs(eq.len() - 2, &Part::One) {
            if eq[0]
                == eq
                    .iter()
                    .skip(2)
                    .zip(op_seq.iter())
                    .fold(eq[1], |res, (next_val, operator)| match operator {
                        '+' => res + next_val,
                        '*' => res * next_val,
                        _ => panic!(),
                    })
            {
                part1 += eq[0];
                break;
            }
        }

        for op_seq in prep_op_seqs(eq.len() - 2, &Part::Two) {
            if eq[0]
                == eq
                    .iter()
                    .skip(2)
                    .zip(op_seq.iter())
                    .fold(eq[1], |res, (next, op)| match op {
                        '+' => res + next,
                        '*' => res * next,
                        '|' => (res.to_string() + &next.to_string()).parse().unwrap(),
                        _ => panic!(),
                    })
            {
                part2 += eq[0];
                break;
            }
        }
    }

    println!("part1: {part1} part2: {part2}");
}
