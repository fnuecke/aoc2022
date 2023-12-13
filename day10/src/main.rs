use std::io::Result;

use aoc2022::read_lines;

fn emulate<'a, L>(lines: L) -> impl Iterator<Item = (i32, i32)> + 'a
where
    L: Iterator<Item = Result<String>> + 'a,
{
    let mut acc = 1;
    lines.flat_map(move |line| -> Box<dyn Iterator<Item = (i32, i32)>> {
        let inst = line.unwrap();
        if inst == "noop" {
            Box::new([(acc, acc)].into_iter())
        } else if inst.starts_with("addx ") {
            let old_acc = acc;
            let value: i32 = inst["addx ".len()..].parse().unwrap();
            acc += value;
            Box::new([(old_acc, old_acc), (old_acc, acc)].into_iter())
        } else {
            panic!("invalid operation: {}", inst);
        }
    })
}

fn get_signal<'a, A, C>(mut accumulator: A, cycles: C) -> impl Iterator<Item = i32> + 'a
where
    A: Iterator<Item = (i32, i32)> + 'a,
    C: Iterator<Item = usize> + 'a,
{
    cycles.scan(0 as usize, move |current_cycle, cycle| {
        let relative_cycle = cycle - *current_cycle;
        *current_cycle += relative_cycle;
        // 0 indexed, so actual index is cycle number - 1
        accumulator
            .nth(relative_cycle - 1)
            .map(|acc| *current_cycle as i32 * acc.0)
    })
}

fn part1() {
    let accumulator_values = emulate(read_lines("input.txt").unwrap());
    let cycles = vec![20, 60, 100, 140, 180, 220];
    let signals = get_signal(accumulator_values, cycles.into_iter());
    println!("part1: {}", signals.sum::<i32>());
}

fn emulate_crt<'a, C>(cycles: C, width: usize) -> Vec<Vec<char>>
where
    C: Iterator<Item = (i32, i32)> + 'a,
{
    cycles
        .enumerate()
        .map(|(cycle, (signal, _))| {
            let cycle_on_row = (cycle as i32) % width as i32;
            if (signal - 1..=signal + 1).contains(&cycle_on_row) {
                '#'
            } else {
                '.'
            }
        })
        .collect::<Vec<_>>()
        .chunks(width)
        .map(|row| row.to_owned())
        .collect()
}

fn part2() {
    let accumulator_values = emulate(read_lines("input.txt").unwrap());
    let crt = emulate_crt(accumulator_values, 40);
    for row in crt {
        println!("{}", row.iter().collect::<String>());
    }
}

fn main() {
    part1();
    part2();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_cycles() {
        let mut accumulator_values = emulate(read_lines("input_test1.txt").unwrap());
        assert_eq!(accumulator_values.next().unwrap(), (1, 1));
        assert_eq!(accumulator_values.next().unwrap(), (1, 1));
        assert_eq!(accumulator_values.next().unwrap(), (1, 4));
        assert_eq!(accumulator_values.next().unwrap(), (4, 4));
        assert_eq!(accumulator_values.next().unwrap(), (4, -1));
    }

    #[test]
    fn many_cycles() {
        let accumulator_values = emulate(read_lines("input_test2.txt").unwrap());
        let cycles = vec![20, 60, 100, 140, 180, 220];
        let signals = get_signal(accumulator_values, cycles.into_iter());

        let expected_signals = vec![420, 1140, 1800, 2940, 2880, 3960];
        signals
            .zip(expected_signals.into_iter())
            .for_each(|(signal, expected)| {
                assert_eq!(signal, expected);
            });
    }

    #[test]
    fn crt() {
        let accumulator_values = emulate(read_lines("input_test2.txt").unwrap());
        let expected_image = vec![
            "##..##..##..##..##..##..##..##..##..##..",
            "###...###...###...###...###...###...###.",
            "####....####....####....####....####....",
            "#####.....#####.....#####.....#####.....",
            "######......######......######......####",
            "#######.......#######.......#######.....",
        ];
        let crt_image = emulate_crt(accumulator_values, 40);

        crt_image
            .into_iter()
            .zip(expected_image.into_iter())
            .for_each(|(result, expected)| {
                assert_eq!(result.iter().collect::<String>(), expected);
            });
    }
}
