use rayon::prelude::*;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::path::Path;

fn convert(bits: &[u8]) -> u64 {
    bits.iter()
        .fold(0, |result, &bit| {
            (result << 1) ^ bit as u64
        })
}

fn get_set_bit_count(numbers: &Vec<Vec<u8>>) -> (u32, Vec<u32>) {
    let mut bit_count = vec![0; numbers[0].len()];
    let mut number_count = 0;
    for number in numbers {
        number_count += 1;
        for (i, bit) in number.iter().enumerate() {
            bit_count[i] += *bit as u32;
        }
    }
    (number_count, bit_count)
}

fn get_oxygen_generator_rating(number_count: u32, bit_count: &Vec<u32>) -> Vec<u8> {
    bit_count
        .iter()
        .map(|val| if val >= &((&number_count + 1)/ 2) { 1 } else { 0 })
        .collect::<Vec<u8>>()
}

fn get_co2_scrubber_rating(number_count: u32, bit_count: &Vec<u32>) -> Vec<u8> {
    bit_count
        .iter()
        .map(|val| if val < &((&number_count + 1)/ 2) { 1 } else { 0 })
        .collect::<Vec<u8>>()
}

pub fn problem_6() -> io::Result<u64> {
    let path_to_read = Path::new("./src/day-3-input.txt");
    let mut file = fs::File::open(&path_to_read)?;
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)?;

    let numbers: Vec<Vec<u8>> = file_contents
        .par_lines()
        .map(|line| -> Vec<u8> {
            line.chars()
                .filter(|ch| {
                    match ch {
                        '0' => true,
                        '1' => true,
                        _ => false
                    }
                })
                .map(|ch| match ch {
                    '0' => 0,
                    '1' => 1,
                    _ => 0,
                })
                .collect()
        })
        .collect();
    
    let (_, bit_count) = get_set_bit_count(&numbers);

    let length = bit_count.len();
    let mut filtered_oxygen_values = numbers.clone();
    for i in 0..length {
        if filtered_oxygen_values.len() == 1 { break; }
        let (number_count, bit_count) = get_set_bit_count(&filtered_oxygen_values);
        let oxygen_generator_rating = get_oxygen_generator_rating(number_count, &bit_count);
        let bit = oxygen_generator_rating[i];
        filtered_oxygen_values = filtered_oxygen_values.into_iter().filter(|number| {
            number[i] == bit
        }).collect::<Vec<Vec<u8>>>();
    }

    let mut filtered_co2_values = numbers.clone();
    for i in 0..length {
        if filtered_co2_values.len() == 1 { break; }
        let (number_count, bit_count) = get_set_bit_count(&filtered_co2_values);
        let co2_scrubber_rating = get_co2_scrubber_rating(number_count, &bit_count);
        let bit = co2_scrubber_rating[i];
        filtered_co2_values = filtered_co2_values.into_iter().filter(|number| {
            number[i] == bit
        }).collect::<Vec<Vec<u8>>>();

    }

    let mut oxygen_rating_vec = filtered_oxygen_values.pop().unwrap();
    let mut co2_rating_vec = filtered_co2_values.pop().unwrap();
    let oxygen_rating_value = convert(oxygen_rating_vec.as_mut_slice());
    let co2_rating_value = convert(co2_rating_vec.as_mut_slice());

    Ok(oxygen_rating_value*co2_rating_value)
}
