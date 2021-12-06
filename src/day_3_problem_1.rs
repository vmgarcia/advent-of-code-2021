use rayon::prelude::*;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::path::Path;

pub fn day_3_problem_1() -> io::Result<u64> {
    let path_to_read = Path::new("./src/day-3-input.txt");
    let mut file = fs::File::open(&path_to_read)?;
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)?;
    let test = file_contents
        .par_lines()
        .map(|line| -> (u64, Vec<u64>) {
            (
                1,
                line.chars()
                    .map(|ch| match ch {
                        '0' => 0,
                        '1' => 1,
                        _ => 0,
                    })
                    .collect(),
            )
        })
        .reduce(
            || (0, Vec::new()),
            |(count_x, x), (count_y, y)| {
                if x.len() == 0 {
                    (count_y, y)
                } else if y.len() == 0 {
                    (count_x, x)
                } else {
                    let mut res = Vec::new();
                    for (i, vx) in x.iter().enumerate() {
                        res.push(vx + y.get(i).unwrap());
                    }
                    (count_x + count_y, res)
                }
            },
        );
    let (count, sums) = test;
    let bin_string = sums
        .iter()
        .map(|val| if val >= &(&count / 2) { "1" } else { "0" })
        .collect::<String>();
    let gamma = u64::from_str_radix(&bin_string, 2).unwrap();
    let complement = bin_string
        .chars()
        .map(|ch| match ch {
            '1' => '0',
            '0' => '1',
            _ => '0',
        })
        .collect::<String>();
    let epsilon = u64::from_str_radix(&complement, 2).unwrap();
    Ok(gamma * epsilon)
}
