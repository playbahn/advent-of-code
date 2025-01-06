use std::collections::VecDeque;

const INPUT: &str = "input/day-09.txt";

/// Lookup Table for constant time checksumming of each file.
/// https://cprimozic.net/blog/optimizing-advent-of-code-2024/#constant-time-checksumming
#[allow(clippy::identity_op)]
const LUT: [usize; 10] = [
    0,
    0,
    0 + 1,
    0 + 1 + 2,
    0 + 1 + 2 + 3,
    0 + 1 + 2 + 3 + 4,
    0 + 1 + 2 + 3 + 4 + 5,
    0 + 1 + 2 + 3 + 4 + 5 + 6,
    0 + 1 + 2 + 3 + 4 + 5 + 6 + 7,
    0 + 1 + 2 + 3 + 4 + 5 + 6 + 7 + 8,
];

/// Any file.
#[derive(Debug)]
struct File {
    /// file ID
    id: usize,
    /// Blocks left of this file ID that can be moved to the front
    blocks: usize,
    /// Block offset on `Disk` (for Part 2)
    offset: usize,
}

impl File {
    fn new(id: usize, blocks: usize, offset: usize) -> Self {
        Self { id, blocks, offset }
    }
}

/// Reader on actual disk map.
#[derive(Debug)]
struct Reader {
    /// The position of reader on the actual disk map.
    pos: usize,
    /// No. of blocks the file is consuming (if `pos` is even),
    /// or the no. of empty blocks (if `pos` is odd).
    blocks: usize,
}

impl Reader {
    fn new(pos: usize, blocks: usize) -> Self {
        Self { pos, blocks }
    }
}

fn main() {
    let t1 = std::time::Instant::now();

    let disk_map = std::fs::read_to_string(INPUT).unwrap();
    let disk_map = disk_map.trim_end();

    let mut disk_reader = disk_map
        .char_indices()
        .map(|(pos, blocks)| Reader::new(pos, blocks.to_digit(10).unwrap() as usize));

    let mut files = disk_map
        .char_indices()
        .rev()
        .step_by(2)
        .map(|(id, blocks)| File::new(id >> 1, blocks.to_digit(10).unwrap() as usize, 0));

    // Position of where the writer would be after any operation.
    let mut fragger: usize = 0;
    // dummy vars
    let mut file = File::new(usize::MAX, 0, usize::MAX);
    let mut read = Reader::new(usize::MAX, 0);
    let mut p1: usize = 0;

    loop {
        if read.blocks == 0 {
            match disk_reader.next() {
                Some(next) => read = next,
                None => break,
            }
        }
        if file.blocks == 0 {
            match files.next() {
                Some(next) => file = next,
                None => break,
            }
        }

        // wrong (causes errors when `reader.pos` is odd) ---
        // `reader.pos >> 1 > file.id`
        // right ---
        if read.pos > file.id << 1 {
            break;
        }

        if read.pos & 0b1 == 0 {
            let read_blocks = if read.pos >> 1 == file.id {
                file.blocks
            } else {
                read.blocks
            };

            p1 += (read_blocks * fragger + LUT[read_blocks]) * (read.pos >> 1);
            fragger += read_blocks;
            read.blocks = 0;
        } else {
            match read.blocks.cmp(&file.blocks) {
                std::cmp::Ordering::Less => {
                    p1 += (read.blocks * fragger + LUT[read.blocks]) * file.id;
                    fragger += read.blocks;
                    file.blocks -= read.blocks;
                    read.blocks = 0;
                }
                std::cmp::Ordering::Equal => {
                    p1 += (read.blocks * fragger + LUT[read.blocks]) * file.id;
                    fragger += read.blocks;
                    (file.blocks, read.blocks) = (0, 0);
                }
                std::cmp::Ordering::Greater => {
                    p1 += (file.blocks * fragger + LUT[file.blocks]) * file.id;
                    fragger += file.blocks;
                    read.blocks -= file.blocks;
                    file.blocks = 0;
                }
            }
        }
    }

    println!("p1: {p1} t1: {:?}", t1.elapsed());

    let t2 = std::time::Instant::now();

    // K = free spaces span; V = offsets of free spaces.
    let mut free_spaces: [VecDeque<usize>; 10] = [const { VecDeque::new() }; 10];
    let mut files: Vec<File> = Vec::with_capacity(10000);
    let mut offset = 0usize;

    disk_map.char_indices().for_each(|(index, blocks)| {
        let blocks = blocks.to_digit(10).unwrap() as usize;

        if index & 0b1 == 0 {
            files.push(File::new(index >> 1, blocks, offset));
        } else if blocks > 0 {
            free_spaces[blocks].push_back(offset);
        }

        offset += blocks;
    });

    let mut p2: usize = 0;

    while let Some(file) = files.pop() {
        let mut new_offset = file.offset;
        let mut index = file.blocks;

        (file.blocks..10).for_each(|blocks| {
            if let Some(offset) = free_spaces[blocks].front() {
                if *offset < new_offset {
                    new_offset = *offset;
                    index = blocks;
                }
            }
        });

        p2 += (file.blocks * new_offset + LUT[file.blocks]) * file.id;

        let old_offset = free_spaces[index].pop_front();
        if index > file.blocks {
            free_spaces[index - file.blocks].push_front(old_offset.unwrap() + file.blocks);
            free_spaces[index - file.blocks].make_contiguous().sort();
        }
    }

    println!("p2: {p2} t2: {:?}", t2.elapsed());
}
