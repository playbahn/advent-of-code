const INPUT: &str = "input/day-09.txt";

/// Any file.
struct File {
    /// file ID
    id: usize,
    /// Blocks left of this file ID that can be moved to the front
    to_move: usize,
}

/// Reader on fragmented disk map.
struct Reader {
    /// The position of reader on the actual fragmented disk map.
    pos: usize,
    /// No. of blocks the file is consuming (if `pos` is even),
    /// or the no. of empty blocks (if `pos` is odd).
    remaining: usize,
}

fn main() {
    let start = std::time::Instant::now();
    let disk = std::fs::read_to_string(INPUT).unwrap();
    let disk = disk.trim_end();

    let mut disk_reader = disk.char_indices().map(|ch| Reader {
        pos: ch.0,
        remaining: ch.1.to_digit(10).unwrap() as usize,
    });

    let last_id: usize = disk.len() >> 1;
    let mut files = disk
        .chars()
        .rev()
        .step_by(2)
        .enumerate()
        .map(|(file_id, blocks)| File {
            id: last_id - file_id,
            to_move: blocks.to_digit(10).unwrap() as usize,
        });

    // Position of where the defragging cursor would be after any operation.
    let mut dcp: usize = 0;
    let mut part1: usize = 0;
    let mut file = files.next().unwrap();
    let mut reader = disk_reader.next().unwrap();

    loop {
        if reader.remaining == 0 {
            reader = disk_reader.next().unwrap();
        }

        if file.to_move == 0 {
            file = files.next().unwrap();
        }

        if reader.pos >> 1 > file.id {
            break;
        }

        if reader.pos & 0b1 == 0 {
            // At a file
            let to_read = if reader.pos >> 1 == file.id {
                file.to_move
            } else {
                reader.remaining
            };

            part1 += (dcp..dcp + to_read).sum::<usize>() * (reader.pos >> 1);
            dcp += to_read;
            reader.remaining -= reader.remaining;
        } else {
            // At free space
            match reader.remaining.cmp(&file.to_move) {
                std::cmp::Ordering::Less => {
                    part1 += (dcp..dcp + reader.remaining).sum::<usize>() * file.id;
                    dcp += reader.remaining;
                    file.to_move -= reader.remaining;
                    reader.remaining = 0;
                }
                std::cmp::Ordering::Equal => {
                    part1 += (dcp..dcp + reader.remaining).sum::<usize>() * file.id;
                    dcp += reader.remaining;
                    (file.to_move, reader.remaining) = (0, 0);
                }
                std::cmp::Ordering::Greater => {
                    part1 += (dcp..dcp + file.to_move).sum::<usize>() * file.id;
                    dcp += file.to_move;
                    reader.remaining -= file.to_move;
                    file.to_move = 0;
                }
            }
        }
    }

    println!("{}s", start.elapsed().as_secs_f64());
    println!("part1: {part1}");
}
