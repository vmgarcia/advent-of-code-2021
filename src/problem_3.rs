use std::fs;
use std::io;
use std::io::prelude::*;
use std::path::Path;

pub fn problem_3() -> io::Result<u64> {
    let path_to_read = Path::new("./src/day-2-input.txt");
    let file = fs::File::open(&path_to_read)?;
    let reader = io::BufReader::new(file);

    let mut position = (0, 0);
    for line_result in reader.lines() {
        let line = line_result?;
        let mut line = line.split_whitespace();
        if let (Some(direction), Some(distance_str)) = (line.next(), line.next()) {
            if let Ok(distance) = distance_str.parse::<u64>() {
                match direction {
                    "forward" => {
                        position.0 += distance;
                    },
                    "down" => {
                        position.1 += distance;
                    },
                    "up" => {
                        position.1 -= distance;
                    },
                    _ => ()
                }
            }
        }
    }

    Ok(position.0 * position.1)
}
