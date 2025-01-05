fn main() {
    let ingredients: [[i16; 5]; 4] = [
        [2, 0, -2, 0, 3], // sprinkles
        [0, 5, -3, 0, 3], // butterscotch
        [0, 0, 5, -1, 8], // chocolate
        [0, -1, 0, 5, 8], // candy
    ];

    let mut i3: i16;
    let mut score: u32;
    let mut max_score: u32 = 0;
    let mut max_score_calories: u32 = 0;
    let (mut capacity, mut durability, mut flavor, mut texture, mut calories);
    for i0 in 0..100i16 {
        // sprinkles amount
        for i1 in 0..100 - i0 {
            // butterscotch amount
            for i2 in 0..100 - i0 - i1 {
                // chocolate amount
                i3 = 100 - i0 - i1 - i2; // candy amount

                capacity = (i0 * ingredients[0][0]
                    + i1 * ingredients[1][0]
                    + i2 * ingredients[2][0]
                    + i3 * ingredients[3][0]) as i32;
                durability = (i0 * ingredients[0][1]
                    + i1 * ingredients[1][1]
                    + i2 * ingredients[2][1]
                    + i3 * ingredients[3][1]) as i32;
                flavor = (i0 * ingredients[0][2]
                    + i1 * ingredients[1][2]
                    + i2 * ingredients[2][2]
                    + i3 * ingredients[3][2]) as i32;
                texture = (i0 * ingredients[0][3]
                    + i1 * ingredients[1][3]
                    + i2 * ingredients[2][3]
                    + i3 * ingredients[3][3]) as i32;
                calories = (i0 * ingredients[0][4]
                    + i1 * ingredients[1][4]
                    + i2 * ingredients[2][4]
                    + i3 * ingredients[3][4]) as i32;

                if capacity > 0 && durability > 0 && flavor > 0 && texture > 0 {
                    score = (capacity * durability * flavor * texture) as u32;
                } else {
                    continue;
                }

                if calories == 500 {
                    max_score_calories = max_score_calories.max(score);
                }
                max_score = max_score.max(score);
            }
        }
    }

    println!("{}", max_score);
    println!("{}", max_score_calories);
}
