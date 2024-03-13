use std::cmp::Ordering;

fn main() {
    let weights: [u8; 28] = [
        1, 3, 5, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 67, 71, 73, 79, 83, 89, 97,
        101, 103, 107, 109, 113,
    ];

    // const GRP_W: u16 = 1524 / 3;
    // PART 2
    const GRP_W: u16 = 1524 / 4;
    let mut group1: Vec<u8> = Vec::from_iter(weights.into_iter());
    let mut sum: u16;
    let mut least_qe: u128 = u128::MAX;
    // println!("{}", least_qe);
    let mut cur_qe: u128;

    for k in 5..10 {
        'package_combination: for package_combination in
            dinglebit_combinatorics::Combination::new(28, k)
        {
            sum = 0;
            cur_qe = 1;
            for idx in &package_combination {
                sum += weights[*idx] as u16;
                if sum > GRP_W {
                    continue 'package_combination;
                }
                cur_qe *= weights[*idx] as u128;
                if cur_qe > least_qe {
                    continue 'package_combination;
                }
            }

            match (
                sum,
                package_combination.len().cmp(&group1.len()),
                cur_qe.cmp(&least_qe),
            ) {
                (GRP_W, Ordering::Less, _) => {
                    group1.truncate(package_combination.len());
                    for (idxg, idxw) in package_combination.iter().enumerate() {
                        group1[idxg] = weights[*idxw];
                    }
                }
                (GRP_W, Ordering::Equal, Ordering::Less) => {
                    least_qe = cur_qe;
                    for (idxg, idxw) in package_combination.iter().enumerate() {
                        group1[idxg] = weights[*idxw];
                    }
                }
                _ => (),
            }
        }
    }
    println!("{}", least_qe);
}
