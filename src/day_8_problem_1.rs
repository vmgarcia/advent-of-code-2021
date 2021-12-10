use std::fs;
use std::io;
use std::io::prelude::*;
use std::path::Path;

fn get_digit_from_segment_count(segment_count: usize) -> Option<u32> {
    match segment_count {
        2 => Some(1),
        3 => Some(7),
        4 => Some(4),
        7 => Some(8),
        _ => None,
    }
}

pub fn day_8_problem_1() -> io::Result<u64> {
    let path_to_read = Path::new("./src/day-8-input.txt");
    let mut file = fs::File::open(&path_to_read)?;
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)?;

    let mut count = 0;

    for line in file_contents.lines() {
        let (_, end) = line.split_once(" | ").unwrap();
        let end_split = end.split_whitespace();
        for code in end_split {
            if let Some(digit) = get_digit_from_segment_count(code.len()) {
                count += 1;
            }
        }
    }

    Ok(count)
}
