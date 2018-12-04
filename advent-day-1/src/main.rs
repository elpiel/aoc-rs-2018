use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader};

fn main () {
    let input_path = Path::new("advent-day-1/input_frequencies.txt");
    let file_f: File = File::open(input_path)
        .expect("File could not be read");
    let file = BufReader::new(file_f);

    let result_part_1 = file.lines().fold(0, |mut acc, line|{
        let line_num: i32 = line.unwrap().parse().unwrap();
        acc += line_num;
        acc
    });

    println!("Part 1: {}", result_part_1);
}