use std::collections::VecDeque;

const INPUT: &str = "input/day-09.txt";

/// Any file.
#[derive(Debug)]
struct File {
    /// file ID
    id: usize,
    /// Blocks left of this file ID that can be moved to the front
    blocks: u8,
    /// Block offset on `Disk` (for Part 2)
    offset: usize,
}

/// Reader on actual disk map.
#[derive(Debug)]
struct Reader {
    /// The position of reader on the actual disk map.
    pos: usize,
    /// No. of blocks the file is consuming (if `pos` is even),
    /// or the no. of empty blocks (if `pos` is odd).
    blocks: u8,
}

#[derive(Clone)]
struct Block(Option<u16>);

struct Disk(Vec<Block>);

impl Disk {
    fn swap_blocks(&mut self, to: usize, from: usize, count: u8) {
        (0..count.into()).for_each(|offset| self.swap(to + offset, from + offset));
    }
}

fn main() {
    let p1start = std::time::Instant::now();

    let disk_map = std::fs::read_to_string(INPUT).unwrap();
    let disk_map = disk_map.trim_end();

    let mut disk_reader = disk_map.char_indices().map(|ch| Reader {
        pos: ch.0,
        blocks: ch.1.to_digit(10).unwrap() as u8,
    });

    let last_id = disk_map.len() >> 1;
    let mut files = (disk_map.chars().rev().step_by(2))
        .enumerate()
        .map(|(file_id, blocks)| File {
            id: last_id - file_id,
            blocks: blocks.to_digit(10).unwrap() as u8,
            // not reqd for Part 1
            offset: 0,
        });

    // Position of where the writer would be after any operation.
    let mut writer_pos: usize = 0;
    let mut file = files.next().unwrap();
    let mut reader = disk_reader.next().unwrap();
    let mut part1: usize = 0;

    loop {
        if reader.blocks == 0 {
            if let Some(next) = disk_reader.next() {
                reader = next
            } else {
                break;
            };
        }
        if file.blocks == 0 {
            if let Some(next) = files.next() {
                file = next
            } else {
                break;
            };
        }

        println!("{reader:?}");
        // wrong: causes errors when `reader.pos` is odd
        // `reader.pos >> 1 > file.id`
        // right:
        if reader.pos > file.id << 1 {
            break;
        }

        if reader.pos & 0b1 == 0 {
            let to_read = if reader.pos >> 1 == file.id {
                file.blocks
            } else {
                reader.blocks
            }
            as usize;

            part1 += (writer_pos..writer_pos + to_read).sum::<usize>() * (reader.pos >> 1);
            writer_pos += to_read;
            reader.blocks = 0;
        } else {
            match reader.blocks.cmp(&file.blocks) {
                std::cmp::Ordering::Less => {
                    part1 +=
                        (writer_pos..writer_pos + reader.blocks as usize).sum::<usize>() * file.id;
                    writer_pos += reader.blocks as usize;
                    file.blocks -= reader.blocks;
                    reader.blocks = 0;
                }
                std::cmp::Ordering::Equal => {
                    part1 += (writer_pos..writer_pos + reader.blocks as usize).sum::<usize>() * file.id;
                    writer_pos += reader.blocks as usize;
                    (file.blocks, reader.blocks) = (0, 0);
                }
                std::cmp::Ordering::Greater => {
                    part1 += (writer_pos..writer_pos + file.blocks as usize).sum::<usize>() * file.id;
                    writer_pos += file.blocks as usize;
                    reader.blocks -= file.blocks;
                    file.blocks = 0;
                }
            }
        }
    }

    println!("{}s", p1start.elapsed().as_secs_f64());
    println!("part1: {part1}");

    let p2start = std::time::Instant::now();

    // K = free spaces length/span; V = starting positions (offsets) of free spaces.
    let mut free_spaces: [VecDeque<usize>; 10] = [const { VecDeque::new() }; 10];
    let mut disk_files: [VecDeque<usize>; 10] = [const { VecDeque::new() }; 10];
    let mut files: Vec<File> = Vec::with_capacity(10000);
    let mut offset = 0usize;
    let mut disk: Disk = Disk(Vec::with_capacity(100000));

    disk_map.char_indices().for_each(|(index, blocks)| {
        let blocks = blocks.to_digit(10).unwrap() as usize;

        if index & 0b1 == 0 {
            disk_files[blocks].push_back(offset);
            let id = index >> 1;
            disk.extend(std::iter::repeat(Block(Some((index >> 1) as u16))).take(blocks));
            files.push(File {
                id: index >> 1,
                blocks: blocks as u8,
                offset,
            });
        } else if blocks != 0 {
            free_spaces[blocks].push_back(offset);
            disk.extend(std::iter::repeat(Block(None)).take(blocks));
        }

        offset += blocks;
    });

    // while let Some(x) = files.pop() {
    //     println!("{x:?}");
    // }

    // println!("disk_files: {disk_files:?}");
    // println!("free_spaces: {free_spaces:?}");
    // println!("{disk:?}");

    println!("{}s", p2start.elapsed().as_secs_f64());
}

impl std::ops::Deref for Block {
    type Target = Option<u16>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Block {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::fmt::Debug for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            Some(id) => write!(f, "{id}"),
            None => write!(f, "_"),
        }
    }
}

impl std::ops::Deref for Disk {
    type Target = Vec<Block>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Disk {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::fmt::Debug for Disk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
