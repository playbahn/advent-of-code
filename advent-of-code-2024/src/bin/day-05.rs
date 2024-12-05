fn main() {
    let input = std::fs::read_to_string("input/day-05.txt").unwrap();

    // for any `rule` in `rules`, `rule.0` comes before `rule.1`
    let rules: Vec<(&str, &str)> = input.split_once("\n\n").unwrap().0.lines().fold(
        Vec::with_capacity(1176),
        |mut rules, rule| {
            rules.push((&rule[..2], &rule[3..]));
            rules
        },
    );

    let mut updates: Vec<Vec<&str>> = input.split_once("\n\n").unwrap().1.lines().fold(
        Vec::with_capacity(192),
        |mut updates, update| {
            updates.push(Vec::from_iter(update.split(',')));
            updates
        },
    );

    let mut sum1 = 0u16;
    let mut sum2 = 0u16;
    let mut correct;

    '_update: for update in &mut updates {
        correct = true;
        // for any index, say `2` in an `update` of `len() = 5`, we check
        // if pages at indices `3` or `4` come before page at index `2`
        // in `rules`. If so, `update` is in incorrect order.
        for came_before in 0..update.len() - 1 {
            for came_after in came_before + 1..update.len() {
                // page `update[came_after]` comes AFTER
                // page `update[came_before]` in current update
                if rules.contains(&(update[came_after], update[came_before])) {
                    // page `update[came_after]` comes BEFORE
                    // page `update[came_before]` in rules;
                    // update is in incorrect order.
                    correct = false;
                    update.swap(came_after, came_before);
                }
            }
        }

        if correct {
            sum1 += update[update.len() >> 1].parse::<u16>().unwrap();
        } else {
            sum2 += update[update.len() >> 1].parse::<u16>().unwrap();
        }
    }

    println!("sum1: {sum1} sum2: {sum2}")
}
