use aoc_day_three::*;
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

    let input_box_ids = file.lines().map(|line|{
        line.unwrap()
    }).collect();

    println!("Part 1: {}", calculate_checksum(&input_box_ids));
    println!("Part 2: {}", find_common_correct_box_id_part(&input_box_ids));
}
