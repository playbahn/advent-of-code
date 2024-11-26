use std::fs;

fn main() {
    let triangles: String = fs::read_to_string("input/day-03.txt").unwrap();
    // PART 1
    // let possible: u16 = triangles.lines().fold(0, |possible, dim| {
    //     let mut dim = dim.trim().split_ascii_whitespace();
    //     let _1: u16 = dim.next().unwrap().parse().unwrap();
    //     let _2: u16 = dim.next().unwrap().parse().unwrap();
    //     let _3: u16 = dim.next().unwrap().parse().unwrap();
    //     if _1 + _2 > _3 && _2 + _3 > _1 && _3 + _1 > _2 {
    //         possible + 1
    //     } else {
    //         possible
    //     }
    // });

    // PART 2
    let (possible, _, _, _): (u16, _, _, _) = triangles.lines().fold(
        (0, Vec::new(), Vec::new(), Vec::new()),
        |(mut possible, mut _1, mut _2, mut _3), dim| {
            let mut dim = dim.trim().split_ascii_whitespace();
            let _1_side: u16 = dim.next().unwrap().parse().unwrap();
            let _2_side: u16 = dim.next().unwrap().parse().unwrap();
            let _3_side: u16 = dim.next().unwrap().parse().unwrap();

            if _1.len() == 2 {
                if _1[0] + _1[1] > _1_side && _1[1] + _1_side > _1[0] && _1_side + _1[0] > _1[1] {
                    possible += 1;
                }
                _1.clear();
                if _2[0] + _2[1] > _2_side && _2[1] + _2_side > _2[0] && _2_side + _2[0] > _2[1] {
                    possible += 1;
                }
                _2.clear();
                if _3[0] + _3[1] > _3_side && _3[1] + _3_side > _3[0] && _3_side + _3[0] > _3[1] {
                    possible += 1;
                }
                _3.clear();
            } else {
                _1.push(_1_side);
                _2.push(_2_side);
                _3.push(_3_side);
            }

            (possible, _1, _2, _3)
        },
    );

    println!("{}", possible);
}
