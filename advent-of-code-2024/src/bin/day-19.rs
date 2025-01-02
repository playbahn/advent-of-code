use std::collections::{HashMap, HashSet};

const INPUT: &str = "input/day-19.txt";

fn ways<'a>(design: &'a str, pats: &HashSet<&str>, cache: &mut HashMap<&'a str, usize>) -> usize {
    if design.is_empty() {
        return 1;
    }

    if let Some(&count) = cache.get(design) {
        return count;
    }

    let count = pats
        .iter()
        .filter(|pat| design.starts_with(*pat))
        .fold(0, |count, pat| {
            count + ways(design.strip_prefix(pat).unwrap(), pats, cache)
        });

    cache.insert(design, count);

    count
}

fn main() {
    let start = std::time::Instant::now();
    let input = std::fs::read_to_string(INPUT).unwrap();
    let (patterns, designs) = input.split_once("\n\n").unwrap();
    let pats: HashSet<&str> = patterns.split(", ").collect();

    let mut cache: HashMap<&str, usize> = HashMap::new();

    let (part1, part2) = designs.lines().fold((0, 0), |(possible, total), design| {
        let ways = ways(design, &pats, &mut cache);
        (possible + ways.clamp(0, 1), total + ways)
    });

    let end = start.elapsed().as_secs_f64();
    println!("part1: {part1}\npart2: {part2}");
    println!("{end}s")
}

// was used for Part 1
const _PATTERN_UB: usize = 8 + 1;
fn _possible(design: &str, patterns: &HashSet<String>) -> bool {
    for stripes in 1.._PATTERN_UB.min(design.len()) {
        if patterns.contains(&design[..stripes])
            && (patterns.contains(&design[stripes..]) || _possible(&design[stripes..], patterns))
        {
            return true;
        }
    }

    false
}
