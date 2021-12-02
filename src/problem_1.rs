use std::fs;
use std::io;
use std::io::prelude::*;
use std::path::Path;

pub fn problem_1() -> io::Result<u64> {
    let path_to_read = Path::new("./src/problem-1-input.txt");
    let file = fs::File::open(&path_to_read)?;
    let reader = io::BufReader::new(file);

    let mut increasing_count = 0;
    let mut prev: Option<u64> = None;
    for line_result in reader.lines() {
        let line: String = line_result?;
        if let Ok(num) = line.parse() {
            if let Some(previous_num) = prev {
                if previous_num < num {
                    increasing_count += 1;
                }
            }
            prev = Some(num);
        }
    }
    Ok(increasing_count)
}
