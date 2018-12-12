use aoc_day_one::{calculate_frequency, calculate_first_frequency_twice};
use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader};
use std::env;

fn main () {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please provide a file path!")
    }

    let input_path = Path::new(&args[1]);
    let file_f: File = File::open(input_path)
        .expect("File could not be read or doesn't exist");
    let file = BufReader::new(file_f);

    let result_part_1 = file.lines().map(|line|{
        line.unwrap().parse::<i32>().unwrap()
    }).collect();

    println!("Part 1: {}", calculate_frequency(&result_part_1));
    println!("Part 2: {}", calculate_first_frequency_twice(&result_part_1));
}