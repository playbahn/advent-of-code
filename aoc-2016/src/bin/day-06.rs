#[derive(Eq, Debug)]
struct ChFreq(char, u16);

impl Ord for ChFreq {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.1.cmp(&other.1)
    }
}

impl PartialOrd for ChFreq {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for ChFreq {
    fn eq(&self, other: &Self) -> bool {
        self.1 == other.1
    }
}

fn main() {
    let msg: String = std::fs::read_to_string("input/day-06.txt").unwrap();
    let mut tree: Vec<Vec<ChFreq>> = Vec::with_capacity(8);

    (0..8).for_each(|_| {
        tree.push({
            let mut ch_freqs: Vec<ChFreq> = Vec::with_capacity(26);
            ('a'..='z').for_each(|ch| ch_freqs.push(ChFreq(ch, 0)));
            ch_freqs
        })
    });

    msg.lines().for_each(|line| {
        line.char_indices()
            .for_each(|(idx, ch)| tree[idx][ch as usize - 97].1 += 1)
    });

    (0..8).for_each(|idx| {
        tree[idx].sort();
        println!(
            "{} {}",
            // part1
            tree[idx].last().unwrap().0,
            // part2
            tree[idx].first().unwrap().0
        )
    });
}
