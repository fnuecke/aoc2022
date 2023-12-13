use std::{fs::File, io::{self, BufRead}, path::Path, collections::HashSet};
use itertools::Itertools;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_rucksack(line: &String) -> (HashSet<char>, HashSet<char>) {
    let item_count = line.len();
    assert!(item_count % 2 == 0);
    let compartment_size = item_count / 2;
    (line[..compartment_size].chars().collect(), line[compartment_size..].chars().collect())
}

fn get_priorty(item: &char) -> Result<i32, String> {
    match item {
        'a'..='z' => Ok(*item as i32 - 'a' as i32 + 1),
        'A'..='Z' => Ok(*item as i32 - 'A' as i32 + 1 + 26),
        _ => Err(format!("unsupported item: {}", item)),
    }
}

fn part1() {
    let mut priotity_sum = 0;
    for line in read_lines("input.txt").unwrap() {
        let compartments = parse_rucksack(&line.unwrap());
        let intersect = compartments.0.intersection(&compartments.1);
        let priority: i32 = intersect.map(|item| get_priorty(item).unwrap()).sum();
        priotity_sum += priority;
    }

    println!("part 1: {}", priotity_sum);
}

fn part2() {
    let mut priotity_sum = 0;
    for chunk in &read_lines("input.txt").unwrap().chunks(3) {
        let rucksacks = chunk.map(|items| items.unwrap().chars().collect::<HashSet<_>>());
        let intersect = rucksacks.reduce(|a, b| a.intersection(&b).copied().collect()).unwrap();
        let priorities = intersect.iter().map(|item| get_priorty(item).unwrap());
        let priority = priorities.exactly_one().unwrap();
        priotity_sum += priority;
    }

    println!("part 2: {}", priotity_sum);
}

fn main() {
    part1();
    part2();
}
