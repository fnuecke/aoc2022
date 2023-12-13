fn try_insert(values: &mut Vec<usize>, len: usize, value: usize) {
    if value > 0 {
        values
            .binary_search_by(|it| value.cmp(&it))
            .map_err(|index| values.insert(index, value))
            .ok();
        values.truncate(len);
    }
}

fn get_maxima(filename: &str, len: usize) -> Result<Vec<usize>, Box<dyn std::error::Error>> {
    let mut elves: Vec<usize> = vec![];
    let mut accumulator = 0;
    for line in aoc2022::read_lines(filename)? {
        let value = line?;
        if value.is_empty() {
            try_insert(&mut elves, len, accumulator);
            accumulator = 0;
        } else {
            value
                .parse::<usize>()
                .map(|value| accumulator += value)
                .ok();
        }
    }

    try_insert(&mut elves, len, accumulator);

    Ok(elves)
}

fn main() {
    let elves = get_maxima("input.txt", 3).unwrap();

    println!("top elf = {}", elves[0]);
    println!("top 3 elves = {}", elves.iter().sum::<usize>());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sums_are_correct_and_sorted() {
        let elves = get_maxima("test_input.txt", usize::MAX).unwrap();
        assert_eq!(elves.len(), 5);
        assert_eq!(elves[0], 24000);
        assert_eq!(elves[1], 11000);
        assert_eq!(elves[2], 10000);
        assert_eq!(elves[3], 6000);
        assert_eq!(elves[4], 4000);
    }
}
