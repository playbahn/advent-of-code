use std::collections::{HashMap, HashSet};

enum Part {
    One,
    Two,
}

type Coords = (isize, isize); // (x, y)

fn within_bounds(antinode: Coords) -> bool {
    -1 < antinode.0 && -1 < antinode.1 && antinode.0 < 50 && antinode.1 < 50
}

fn find_antinodes_aux(node: Coords, dist: Coords, part: &Part) -> Vec<Coords> {
    let mut antinodes = if let Part::One = part {
        vec![]
    } else {
        // from https://adventofcode.com/2024/day/8 (part 2):
        //
        // > an antinode occurs at any point that is perfectly in line
        // > with two antennas of the same frequency
        //
        // at any *POINT* - slipped from mind
        //
        // > In fact, the three T-frequency antennas are all exactly
        // > in line with two antennas, so they are all also antinodes!
        //
        // So even if there is no space left on the line crossing `node1`
        // and `node2` in either direction `node1 -> node2` or  `node2 ->
        // node1` that is within map bounds, `node1` is in line with
        // `node1` ITSELF and `node2`, and the same goes for `node2`.
        // BRAH. That was REALLY F***IN HARD to understand.
        vec![node]
    };

    let mut next_antinode = (node.0 + dist.0, node.1 + dist.1);
    loop {
        if within_bounds(next_antinode) {
            antinodes.push(next_antinode);
        } else {
            break;
        }
        if let Part::One = part {
            break;
        }
        next_antinode = (next_antinode.0 + dist.0, next_antinode.1 + dist.1);
    }
    antinodes
}

fn find_antinodes(node1: Coords, node2: Coords, part: Part) -> Vec<Coords> {
    let dist = (node2.0 - node1.0, node2.1 - node1.1);
    // direction: node1 -> node2; distance: positive
    let mut antinodes = find_antinodes_aux(node2, (dist.0, dist.1), &part);
    // direction: node2 -> node1; distance: negative
    antinodes.append(&mut find_antinodes_aux(node1, (-dist.0, -dist.1), &part));
    antinodes
}

fn main() {
    let mut map: HashMap<char, Vec<Coords>> = HashMap::new();
    let mut antinodes: HashSet<Coords> = HashSet::new();

    std::fs::read_to_string("input/day-08.txt")
        .unwrap()
        .char_indices()
        .for_each(|(offset, freq)| {
            if freq != '.' && freq != '\n' {
                // casted to `isize` to steer clear of underflow issues in
                // `find_antinodes`. 50 non-`\n` chars + 1 `\n` every line
                let node = (
                    offset as isize % 51, // x-coordinate
                    offset as isize / 51, // y-coordinate
                );
                map.entry(freq)
                    .and_modify(|nodes| nodes.push(node))
                    .or_insert(vec![node]);
            }
        });

    // part 1
    for nodes in map.values() {
        for node1 in 0..nodes.len() - 1 {
            for node2 in node1 + 1..nodes.len() {
                antinodes.extend(find_antinodes(nodes[node1], nodes[node2], Part::One));
            }
        }
    }

    println!("part1: {:?}", antinodes.len());

    // part 2
    for nodes in map.values() {
        for node1 in 0..nodes.len() - 1 {
            for node2 in node1 + 1..nodes.len() {
                antinodes.extend(find_antinodes(nodes[node1], nodes[node2], Part::Two));
            }
        }
    }

    println!("part2: {:?}", antinodes.len());
}
