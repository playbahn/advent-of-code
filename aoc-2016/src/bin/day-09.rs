// NOTE TO SELF: WHY THE F**K DID YOU WRITE A WHOLE ASS PARSER
// WHEN YOU COULD'VE JUST MULTIPLIED SHIT. OK MAYBE PART 2 IS
// WHERE A PARSER WOULD BE EASIER THAN A MULTIPLICATION ALGO.



fn recursive_decomprssed_len(packed: &str, repeat_packed: u16) -> u32 {
    let markers: Vec<usize> = packed
        .match_indices(['(', 'x', ')'])
        .map(|(idx, _)| idx)
        .collect();
    0
}

fn main() {
    let packed = std::fs::read_to_string("input/day-09.txt").unwrap();
    let mut len1 = 0u32;
    let markers: Vec<usize> = packed
        .match_indices(['(', 'x', ')'])
        .map(|(idx, _)| idx)
        .collect();
    let mut curr_idx = 0usize;
    for marker in markers.chunks(3) {
        if marker[0] < curr_idx {
            continue;
        }
        
        len1 += (marker[0] - curr_idx) as u32;
        curr_idx = marker[2] + 1;
        
        let char_s: u32 = packed[marker[0] + 1..marker[1]].parse().unwrap();
        let repeat: u32 = packed[marker[1] + 1..marker[2]].parse().unwrap();

        len1 += char_s * repeat;
        curr_idx += char_s as usize;
    }

    println!("len1: {len1}");

    let len2: u32;
    
    
    
    // let mut packed_chars: Chars<'_> = packed.chars();

    // // when getting a marker, marked_l_chars holds the marker digits before 'x'
    // // when not getting a marker, marked_l_chars holds the actual characters that it referred to previously
    // let (mut unpacked, mut unpacked_2, mut marker_l_chars) =
    //     (String::new(), String::new(), String::new());
    // // marker_l holds the marker digits before 'x' parsed into u32
    // // marker_r holds the marker digits after 'x' parsed into u32
    // let (mut marker_l, mut marker_r) = (0u32, 0usize);
    // let mut getting_marker = false;

    // // let mut ch_num = 0;

    // let mut unpack = |ch, packed: &mut Chars<'_>| {
    //     // ch_num += 1;
    //     // print!("ch: {ch} : ");
    //     match (ch, getting_marker, marker_l) {
    //         ('(', false, 0) => getting_marker = true,
    //         ('x', true, 0) => {
    //             marker_l = marker_l_chars.parse().unwrap();
    //             marker_l_chars.clear();
    //         }
    //         (')', true, 1..) => getting_marker = false,
    //         (ch, true, 0) => marker_l_chars.push(ch),
    //         (ch, true, 1..) => marker_r = 10 * marker_r + ch.to_digit(10).unwrap() as usize,
    //         (ch, false, 0) => unpacked.push(ch),
    //         (ch, false, 1..) => {
    //             // 1 char pushed here
    //             marker_l_chars.push(ch);
    //             // decrement no. of chars to push by 1
    //             marker_l -= 1;
    //             // (marker_l - 1) chars pushed here
    //             while marker_l > 0 {
    //                 marker_l_chars.push(packed.next().unwrap());
    //                 marker_l -= 1;
    //             }
    //             unpacked.push_str(&marker_l_chars.repeat(marker_r));
    //             marker_l_chars.clear();
    //             marker_r = 0;
    //         }
    //     }
    // };

    // while let Some(ch) = packed_chars.next() {
    //     unpack(ch, &mut packed_chars);
    // }

    // println!("unpacked_1 len: {}", unpacked.len());
}
