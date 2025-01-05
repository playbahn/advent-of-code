fn main() {
    let mut input: [[char; 140]; 140] = [['.'; 140]; 140];

    std::fs::read_to_string("input/day-04.txt")
        .unwrap()
        .lines()
        .enumerate()
        .for_each(|(row, line)| {
            line.char_indices()
                .for_each(|(col, ch)| input[row][col] = ch)
        });

    let mut xmas = 0u16;

    // row-wise
    input.iter().for_each(|row| {
        row.windows(4).for_each(|window| {
            if window == ['X', 'M', 'A', 'S'] || window == ['S', 'A', 'M', 'X'] {
                xmas += 1;
            }
        });
    });

    // col-wise
    (0..140).for_each(|col| {
        (0..137).for_each(|row| {
            if input[row][col] == 'X' /* ↓ */
                && input[row + 1][col] == 'M'
                && input[row + 2][col] == 'A'
                && input[row + 3][col] == 'S'
                || input[row][col] == 'S' /* ↑ */
                    && input[row + 1][col] == 'A'
                    && input[row + 2][col] == 'M'
                    && input[row + 3][col] == 'X'
            {
                xmas += 1;
            }
        });
    });

    // diagonally
    (0..137).for_each(|row| {
        (3..140).for_each(|col| {
            if input[row][col] == 'X' /* ↙ */
                && input[row + 1][col - 1] == 'M'
                && input[row + 2][col - 2] == 'A'
                && input[row + 3][col - 3] == 'S'
                || input[row][col] == 'S' /* ↗ */
                    && input[row + 1][col - 1] == 'A'
                    && input[row + 2][col - 2] == 'M'
                    && input[row + 3][col - 3] == 'X'
            {
                xmas += 1;
            }
        });

        (0..137).for_each(|col| {
            if input[row][col] == 'X' /* ↘ */
                && input[row + 1][col + 1] == 'M'
                && input[row + 2][col + 2] == 'A'
                && input[row + 3][col + 3] == 'S'
                || input[row][col] == 'S' /* ↖ */
                    && input[row + 1][col + 1] == 'A'
                    && input[row + 2][col + 2] == 'M'
                    && input[row + 3][col + 3] == 'X'
            {
                xmas += 1;
            }
        });
    });

    println!("xmas: {xmas}");

    let mut x_mas = 0u16;

    (1..139).for_each(|row| {
        (1..139).for_each(|col| {
            if input[row][col] == 'A'
                && (input[row - 1][col - 1] == 'M' && input[row + 1][col + 1] == 'S' /* ↘ */
                    || input[row - 1][col - 1] == 'S' && input[row + 1][col + 1] == 'M'/* ↖ */)
                && (input[row - 1][col + 1] == 'M' && input[row + 1][col - 1] == 'S' /* ↙ */
                    || input[row - 1][col + 1] == 'S' && input[row + 1][col - 1] == 'M'/* ↗ */)
            {
                x_mas += 1;
            }
        });
    });

    println!("x_mas: {x_mas}");
}
