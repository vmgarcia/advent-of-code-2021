use std::collections::VecDeque;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::path::Path;

pub fn problem_2() -> io::Result<u64> {
    let path_to_read = Path::new("./src/day-1-input.txt");
    let file = fs::File::open(&path_to_read)?;
    let reader = io::BufReader::new(file);

    let mut increasing_count = 0;
    let mut prev: Option<u64> = None;
    let mut window: VecDeque<u64> = VecDeque::new();
    for line_result in reader.lines() {
        let line: String = line_result?;
        if let Ok(num) = line.parse() {
            window.push_back(num);
            if window.len() == 3 {
                let current_sum = window.iter().sum();
                if let Some(previous_sum) = prev {
                    if previous_sum < current_sum {
                        increasing_count += 1;
                    }
                }
                prev = Some(current_sum);
                window.pop_front();
            }
        }
    }
    Ok(increasing_count)
}
