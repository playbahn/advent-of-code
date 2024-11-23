use std::fs;

fn main() {
    let rooms: String = fs::read_to_string("input/day4.txt").unwrap();
    let mut char_freq: [(char, u8); 26] = [('\0', 0); 26];
    let real: u32 = rooms.lines().fold(0, |real, room| {
        let (encrypted, id_chksm): (Vec<&str>, Vec<&str>) = room.split('-').partition(|subslice| {
            subslice.starts_with(|first_char: char| first_char.is_alphabetic())
        });

        let sector_id: u16 = id_chksm[0][..3].parse::<u16>().unwrap();
        let rotate: u8 = (sector_id % 26) as u8;

        print!("{sector_id} - ");

        for part in &encrypted {
            for byte in part.as_bytes() {
                print!("{}", ((*byte - 97 + rotate) % 26 + 97) as char)
            }
            print!(" ");
        }
        println!();

        char_freq = [
            ('a', 0),
            ('b', 0),
            ('c', 0),
            ('d', 0),
            ('e', 0),
            ('f', 0),
            ('g', 0),
            ('h', 0),
            ('i', 0),
            ('j', 0),
            ('k', 0),
            ('l', 0),
            ('m', 0),
            ('n', 0),
            ('o', 0),
            ('p', 0),
            ('q', 0),
            ('r', 0),
            ('s', 0),
            ('t', 0),
            ('u', 0),
            ('v', 0),
            ('w', 0),
            ('x', 0),
            ('y', 0),
            ('z', 0),
        ];

        for part in encrypted {
            for char in part.chars() {
                char_freq[char as usize - 97].1 += 1;
            }
        }

        char_freq.sort_unstable_by(|a, b| {
            if a.1 != b.1 {
                b.1.cmp(&a.1)
            } else {
                a.0.cmp(&b.0)
            }
        });

        if char_freq
            .into_iter()
            .map(|(c, _)| c)
            .take(5)
            .collect::<String>()
            == id_chksm[0][4..id_chksm[0].len() - 1]
        {
            real + sector_id as u32
        } else {
            real
        }
    });

    println!("{real}")
}
