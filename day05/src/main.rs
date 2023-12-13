use std::{fs::File, io::{self, BufRead}, path::Path};
use lazy_static::lazy_static;
use regex::Regex;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    lazy_static! {
        static ref STACK: Regex = Regex::new(r"(?:\[([A-Z])\]| {3}) (?:\[([A-Z])\]| {3}) (?:\[([A-Z])\]| {3}) (?:\[([A-Z])\]| {3}) (?:\[([A-Z])\]| {3}) (?:\[([A-Z])\]| {3}) (?:\[([A-Z])\]| {3}) (?:\[([A-Z])\]| {3}) (?:\[([A-Z])\]| {3})").unwrap();
        static ref OP: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    }

    let mut stacks9000: Vec<Vec<char>> = vec![
        vec![], vec![], vec![], vec![], vec![], vec![], vec![], vec![], vec![],
    ];
    let mut stacks9001: Vec<Vec<char>> = vec![
        vec![], vec![], vec![], vec![], vec![], vec![], vec![], vec![], vec![],
    ];
    for line in read_lines("input.txt").unwrap() {
        let value = line.unwrap();
        STACK.captures(value.as_str()).map(|cap| {
            for i in 0..9 {
                cap.get(i + 1).map(|c| {
                    stacks9000[i].insert(0, c.as_str().chars().next().unwrap());
                    stacks9001[i].insert(0, c.as_str().chars().next().unwrap());
                });
            }
        });

        OP.captures(value.as_str()).map(|cap| {
            let count: usize = cap.get(1).unwrap().as_str().parse().unwrap();
            let from: usize = cap.get(2).unwrap().as_str().parse().unwrap();
            let to: usize = cap.get(3).unwrap().as_str().parse().unwrap();

            for _ in 0..count {
                stacks9000[from - 1].pop().map(|item| stacks9000[to - 1].push(item));
            }

            let mut buffer: Vec<char> = vec![];
            for _ in 0..count {
                stacks9001[from - 1].pop().map(|item| buffer.push(item));
            }
            for _ in 0..count {
                buffer.pop().map(|item| stacks9001[to - 1].push(item));
            }
        });
    }

    println!("top 9000 = {}", (0..9).map(|i| stacks9000[i].last().unwrap_or(&' ')).collect::<String>());
    println!("top 9001 = {}", (0..9).map(|i| stacks9001[i].last().unwrap_or(&' ')).collect::<String>());
}
