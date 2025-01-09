use std::collections::VecDeque;

const INPUT: &str = "input/day-09.txt";

const P1: usize = 6_386_640_365_805;
const P2: usize = 6_423_258_376_982;

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
struct File {
    /// File ID.
    id: usize,
    /// Blocks left of this file that can be moved to the front.
    blocks: usize,
    /// Block offset (for Part 2).
    offset: u32,
}

impl File {
    fn new(id: usize, blocks: usize, offset: u32) -> Self {
        Self { id, blocks, offset }
    }
}

/// Reader on actual disk map.
struct Reader {
    /// The position of reader on the actual disk map.
    pos: usize,
    /// No. of blocks the file is consuming (if `pos` is even),
    /// or the no. of empty blocks to fill (if `pos` is odd).
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
    let mut file = File::new(usize::MAX, 0, u32::MAX);
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
            let read_blocks = match read.pos >> 1 == file.id {
                true => file.blocks,
                false => read.blocks,
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

    let t1 = t1.elapsed();
    println!("p1: {p1} t1: {t1:?}");
    assert_eq!(p1, P1);

    let t2 = std::time::Instant::now();

    // `free_spaces[n]` holds in creasing order, all the offsets of free spaces
    // spanning `n` blocks.
    let mut free_spaces: [VecDeque<u32>; 10] = [const { VecDeque::new() }; 10];
    let mut file_offsets: [u32; 10_000] = [0; 10_000];

    disk_map.char_indices().fold(0, |offset, (index, blocks)| {
        let blocks = blocks.to_digit(10).unwrap();

        if index & 0b1 == 0 {
            file_offsets[index >> 1] = offset;
        } else if blocks != 0 {
            free_spaces[blocks as usize].push_back(offset);
        }

        offset + blocks
    });

    let files = disk_map
        .char_indices()
        // We calculate checksum from the end.
        .rev()
        .step_by(2)
        .map(|(id, blocks)| {
            File::new(
                id >> 1,
                blocks.to_digit(10).unwrap() as usize,
                file_offsets[id >> 1],
            )
        });

    let p2 = files.fold(0, |checksum, file| {
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

        if new_offset != file.offset {
            let old_offset = free_spaces[index].pop_front().unwrap();
            if index != file.blocks {
                free_spaces[index - file.blocks].push_front(old_offset + file.blocks as u32);
                free_spaces[index - file.blocks].make_contiguous().sort();
            }
        }

        checksum + (file.blocks * new_offset as usize + LUT[file.blocks]) * file.id
    });

    let t2 = t2.elapsed();
    println!("p2: {p2} t2: {t2:?}");
    println!("Total: {:?}", t1 + t2);
    assert_eq!(p2, P2);
}
