use aoc_day_one::calculate_frequency;
use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader};

fn main () {
    let input_path = Path::new("advent-day-one/input_frequencies.txt");
    let file_f: File = File::open(input_path)
        .expect("File could not be read");
    let file = BufReader::new(file_f);

    let result_part_1 = file.lines().map(|line|{
        line.unwrap().parse::<i32>().unwrap()
    }).collect();

    println!("Part 1: {}", calculate_frequency(&result_part_1));
}