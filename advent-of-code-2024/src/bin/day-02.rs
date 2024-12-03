fn is_safe(report: &[u8]) -> bool {
    report
        .windows(2)
        .fold(
            (true, report[0].cmp(&report[1])),
            |(safe, prev_ord), adj| -> (bool, std::cmp::Ordering) {
                (
                    safe && prev_ord == adj[0].cmp(&adj[1])
                        && (1..4).contains(&adj[0].abs_diff(adj[1])),
                    prev_ord,
                )
            },
        )
        .0
}

fn main() {
    let reports: Vec<Vec<u8>> = std::fs::read_to_string("input/day-02.txt")
        .unwrap()
        .lines()
        .map(|report| -> Vec<u8> {
            report
                .split_whitespace()
                .map(|level| level.parse().unwrap())
                .collect()
        })
        .collect();

    let (mut safe1, mut safe2) = (0u32, 0u32);

    'report: for report in reports {
        // part 1
        if is_safe(&report) {
            safe1 += 1;
            safe2 += 1;
            continue;
        }

        // part 2
        for skip_idx in 0..report.len() {
            if is_safe(
                &report
                    .iter()
                    .enumerate()
                    .filter(|(idx, _)| *idx != skip_idx)
                    .map(|(_, level)| *level)
                    .collect::<Vec<u8>>(),
            ) {
                safe2 += 1;
                continue 'report;
            }
        }
    }

    println!("safe1: {safe1} safe2: {safe2}");
}
