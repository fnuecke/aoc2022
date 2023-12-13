use lazy_static::lazy_static;
use regex::Regex;
use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    lazy_static! {
        static ref COMMAND: Regex = Regex::new(r"^\$ (.+)$").unwrap();
        static ref ITEM: Regex = Regex::new(r"^(dir|\d+) (.+)$").unwrap();
    }

    let mut path: Vec<String> = vec![];
    let mut sizes: HashMap<String, usize> = HashMap::new();
    let mut used_size: usize = 0;
    for line in read_lines("input.txt").unwrap() {
        let input = &line.unwrap();
        COMMAND.captures(input.as_str()).map(|cap| {
            let command = cap.get(1).unwrap().as_str();
            let split_index = command.find(' ');
            match split_index {
                Some(idx) => {
                    let (op, arg) = command.split_at(idx);
                    if op == "cd" {
                        if arg.trim() == ".." {
                            path.pop();
                        } else if arg.trim() == "/" {
                            path.clear();
                        } else {
                            path.push(arg.trim().to_string());
                        }
                    } else {
                        panic!("unknown command {}", op);
                    }
                }
                _ => {
                    let op = command;
                    if op == "ls" {
                        // don't care, really...
                    } else {
                        panic!("unknown command {}", op);
                    }
                }
            }
        });
        ITEM.captures(input.as_str()).map(|cap| {
            let dir_or_size = cap.get(1).unwrap().as_str();
            if dir_or_size == "dir" {
                // don't care
            } else {
                let size: usize = dir_or_size.parse().unwrap();
                used_size += size;
                for i in 0..path.len() {
                    let pwd = path
                        .iter()
                        .take(path.len() - i)
                        .cloned()
                        .collect::<Vec<_>>()
                        .join("/");
                    *sizes.entry(pwd).or_insert(0) += size;
                }
            }
        });
    }

    let part1: usize = sizes
        .values()
        .into_iter()
        .filter(|size| **size < 100000)
        .sum();
    println!("part 1: {}", part1);

    const TOTAL: usize = 70000000;
    const REQUIRED: usize = 30000000;
    let current_free = TOTAL - used_size;
    let need_to_free = REQUIRED - current_free;
    let mut best_size = usize::MAX;
    for size in sizes.values() {
        if *size > need_to_free && *size < best_size {
            best_size = *size;
        }
    }
    println!("part 2: {}", best_size);
}
