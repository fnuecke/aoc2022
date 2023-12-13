use std::{fs::File, io::{self, BufRead}, path::Path};
use lazy_static::lazy_static;
use regex::Regex;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    lazy_static! {
        static ref PATTERN: Regex = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    }
    let mut full_overlap_count = 0;
    let mut partial_overlap_count = 0;
    for line in read_lines("input.txt").unwrap() {
        PATTERN.captures(line.unwrap().as_str()).map(|cap| {
            let from0: i32 = cap.get(1).unwrap().as_str().parse().unwrap();
            let until0: i32 = cap.get(2).unwrap().as_str().parse().unwrap();
            let from1: i32 = cap.get(3).unwrap().as_str().parse().unwrap();
            let until1: i32 = cap.get(4).unwrap().as_str().parse().unwrap();
            if from0 >= from1 && until0 <= until1 || from1 >= from0 && until1 <= until0 {
                full_overlap_count += 1;
            }
            if from0 <= until1 && from1 <= until0 || from1 <= until0 && from0 <= until1 {
                partial_overlap_count += 1;
            }
        });
    }

    println!("full overlap count = {}", full_overlap_count);
    println!("partial overlap count = {}", partial_overlap_count);
}
