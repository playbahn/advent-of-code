use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    let input: String = fs::read_to_string("input/day19.txt").unwrap();
    let replacements: HashMap<&str, HashSet<&str>> =
        input.lines().take_while(|line| !line.is_empty()).fold(
            HashMap::new(),
            |mut replacements: HashMap<&str, HashSet<&str>>, replacement: &str| {
                let replacement: Vec<&str> = replacement.split_ascii_whitespace().collect();
                replacements
                    .entry(replacement[0])
                    .and_modify(|results: &mut HashSet<&str>| {
                        results.insert(replacement[2]);
                    })
                    .or_insert(HashSet::from([replacement[2]]));
                replacements
            },
        );

    let molecule: String = String::from(input.lines().last().unwrap());

    let mut distinct: HashSet<String> = HashSet::new();
    let mut new_intermediate: String;

    for (precursor, results) in &replacements {
        for result in results {
            for (idx, _) in molecule.match_indices(precursor) {
                new_intermediate = molecule.clone();
                new_intermediate.replace_range(idx..idx + precursor.len(), result);
                distinct.insert(new_intermediate);
            }
        }
    }

    println!("{}", distinct.len());

    let mut min_steps: usize = usize::MAX;
    devolve_molecule(molecule, 0, &mut min_steps, &replacements);
    println!("{}", min_steps + 1);
}

fn devolve_molecule(
    intermediate: String,
    steps: usize,
    min_steps: &mut usize,
    replacements: &HashMap<&str, HashSet<&str>>,
) {
    if !(steps < *min_steps) {
        return;
    }
    
    // println!("{intermediate}");
    match &intermediate[..] {
        // e => HF  // e => NAl // e => OMg
        "HF" | "NAl" | "OMg" => {
            *min_steps = (*min_steps).min(steps);
            println!("{min_steps}");
        },

        _ => {
            for (precursor, results) in replacements {
                for result in results {
                    for (idx, _) in intermediate.match_indices(result) {
                        let mut new_intermediate: String = intermediate.clone();
                        new_intermediate.replace_range(idx..idx + result.len(), precursor);
                        devolve_molecule(new_intermediate, steps + 1, min_steps, replacements);
                    }
                }
            }
        }
    }
}
