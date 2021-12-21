use regex::Regex;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fs;
use std::io;
use std::io::prelude::*;
use std::path::Path;

struct Dimensions(usize, usize);

fn parse_input(input: String) -> (Vec<u8>, Vec<u8>, Dimensions) {
    let mut input_lines = input.lines();
    let mut image_processing_algorithm = input_lines
        .next()
        .unwrap()
        .chars()
        .map(|ch| if ch == '#' { 1 } else { 0 })
        .collect::<Vec<u8>>();

    input_lines.next(); // dump empty line

    let mut image = Vec::new();

    let first_line = input_lines.next().unwrap();
    let width = first_line.len();

    first_line.chars().for_each(|ch| {
        if ch == '#' {
            image.push(1);
        } else {
            image.push(0);
        }
    });

    let mut height = 1;
    for line in input_lines {
        height += 1;
        line.chars().for_each(|ch| {
            if ch == '#' {
                image.push(1);
            } else {
                image.push(0);
            }
        });
    }

    (image_processing_algorithm, image, Dimensions(width, height))
}

fn get_values_in_block(
    index: usize,
    image: &Vec<u8>,
    Dimensions(width, height): Dimensions,
    iteration: i64
) -> Vec<u8> {
    let get_value = |i: i64| -> u8 {
        if i < 0 || i >= image.len() as i64 {
            (iteration % 2) as u8
        } else {
            *image.get(i as usize).unwrap()
        }
    };

    let i_width = width as i64;
    let transformations = [
        -1 - i_width,
        -i_width,
        1 - i_width,
        -1,
        0,
        1,
        -1 + i_width,
        i_width,
        1 + i_width,
    ];

    transformations
        .into_iter()
        .map(|transform| get_value(index as i64 + transform))
        .collect::<Vec<u8>>()
}

#[test]
fn test_get_values_in_block() {
    let file_contents = String::from("..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#
    
#..#.
#....
##..#
..#..
..###");

    let (_, image, Dimensions(width, height)) = parse_input(file_contents);

    assert_eq!(
        get_values_in_block(12, &image, Dimensions(width, height), 0),
        vec![0, 0, 0, 1, 0, 0, 0, 1, 0]
    );
}

fn convert_bit_vec_to_int(bits: Vec<u8>) -> usize {
    let mut num = 0;
    for bit in bits {
        num = num * 2 + bit as u64;
    }
    num as usize
}

#[test]
fn test_convert_bit_vec_to_int() {
    let int = convert_bit_vec_to_int(vec![0, 0, 0, 1, 0, 0, 0, 1, 0]);
    assert_eq!(int, 34);
}

fn apply_algorithm_to_pixel(
    algorithm: &Vec<u8>,
    image: &Vec<u8>,
    index: usize,
    dimensions: Dimensions,
    iteration: i64
) -> u8 {
    *algorithm
        .get(convert_bit_vec_to_int(get_values_in_block(
            index, image, dimensions, iteration
        )))
        .unwrap()
}

#[test]
fn test_apply_algorithm_to_pixel() {
    let file_contents = String::from("..#.#..#####.#.#.#.###.##.....###.##.f#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#
    
#..#.
#....
##..#
..#..
..###");

    let (image_processing_algorithm, image, Dimensions(width, height)) = parse_input(file_contents);
    assert_eq!(
        apply_algorithm_to_pixel(
            &image_processing_algorithm,
            &image,
            12,
            Dimensions(width, height),
            0
        ),
        1
    );
}

fn pad_image(image: Vec<u8>, Dimensions(width, height): Dimensions, iteration: i64) -> (Vec<u8>, Dimensions) {
    let padded_width = width + 2;
    let padded_height = height + 2;

    let mut padded_image = Vec::with_capacity(padded_width * padded_height);
    for _ in 0..padded_width {
        padded_image.push((iteration % 2) as u8);
    }
    for (i, bit) in image.iter().enumerate() {
        if i % width == 0 {
          padded_image.push((iteration % 2) as u8);
        }
        padded_image.push(*bit);
        if (i + 1) % width == 0 {
          padded_image.push((iteration % 2) as u8);
        }
    }
    for _ in 0..padded_width {
      padded_image.push((iteration % 2) as u8);
    }
    (padded_image, Dimensions(padded_width, padded_height))
}

#[test]
fn test_pad_image() {
    let file_contents = String::from("..#.#..#####.#.#.#.###.##.....###.##.f#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#
    
#..#.
#....
##..#
..#..
..###");

    let (image_processing_algorithm, image, Dimensions(width, height)) = parse_input(file_contents);

    let (image, Dimensions(width, height)) = pad_image(image, Dimensions(width, height), 0);
    print_image(&image, Dimensions(width, height));
}

fn print_image(image: &Vec<u8>, Dimensions(width, height): Dimensions) {
    for i in 0..image.len() {
        if i % width == 0 {
            println!();
        }
        print!(
            "{}",
            if *image.get(i).unwrap() == 1 {
                "#"
            } else {
                "."
            }
        );
    }
    println!();
    println!("--------------");
}

pub fn day_20_problem_1() -> io::Result<u64> {
    let path_to_read = Path::new("./src/day-20-input.txt");
    let mut file = fs::File::open(&path_to_read)?;
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)?;
//     file_contents = String::from("..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#
    
// #..#.
// #....
// ##..#
// ..#..
// ..###");

    let (image_processing_algorithm, image, Dimensions(width, height)) = parse_input(file_contents);

    let mut new_image = image;
    let mut new_width = width;
    let mut new_height = height;
    // print_image(&new_image, Dimensions(new_width, new_height));
    // println!("{:?}", image_processing_algorithm);
    for i in 0..50 {

      let (new_padded_image, Dimensions(new_padded_width, new_padded_height)) =
        pad_image(new_image, Dimensions(new_width, new_height), i);
        new_image = new_padded_image;
        new_width = new_padded_width;
        new_height = new_padded_height;

        new_image = (0..new_image.len())
            .map(|index| {
                apply_algorithm_to_pixel(
                    &image_processing_algorithm,
                    &new_image,
                    index,
                    Dimensions(new_width, new_height),
                    i
                )
            })
            .collect::<Vec<u8>>();

    }

    // println!("{:?}, {}", new_image, new_image.len());
    let mut sum: u64 = 0;
    new_image.into_iter().for_each(|v| sum += v as u64);
    Ok(sum)
}
