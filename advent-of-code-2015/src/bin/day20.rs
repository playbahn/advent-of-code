fn main() {
    const PRESENTS: u32 = 36_000_000;
    // // PART 1
    // let mut houses: Vec<u32> = [1].repeat(1 + PRESENTS as usize / 10);
    // PART 2
    let mut houses: Vec<u32> = [11].repeat(1 + PRESENTS as usize / 11);
    houses[0] = 0;

    for elf in 2..houses.len() {
        // // PART 1
        // for house in (elf..houses.len()).step_by(elf) {
        //     houses[house] += elf as u32;
        // }
        // PART2
        for house in (elf..houses.len()).step_by(elf).take(50) {
            houses[house] += 11 * elf as u32;
        }
    }
    for house in 1..houses.len() {
        // // PART 1
        // if houses[house] >= PRESENTS / 10 {
        // PART 2
        if houses[house] >= PRESENTS {
            println!("{}", house);
            break;
        }
    }
}
