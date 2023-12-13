use aoc2022::{read_non_empty_lines, ParseError};
use lazy_static::lazy_static;
use regex::Regex;

type MonkeyId = usize;
type ItemId = usize;
type WorryLevel = usize;

struct Monkey {
    id: usize,
    items: Vec<ItemId>,
    operation: Box<dyn Fn(WorryLevel) -> WorryLevel>,
    test: Box<dyn Fn(WorryLevel) -> MonkeyId>,
}

fn parse_starting_items<R>(reader: &mut R) -> Result<Vec<ItemId>, Box<dyn std::error::Error>>
where
    R: Iterator<Item = String>,
{
    lazy_static! {
        static ref STARTING_ITEMS: Regex = Regex::new(r"^  Starting items: (.+)$").unwrap();
    };

    let line = reader.next().unwrap();
    let capture = STARTING_ITEMS.captures(line.as_str()).unwrap();
    Ok(capture
        .get(1)
        .unwrap()
        .as_str()
        .split(", ")
        .map(|item| item.parse::<ItemId>().unwrap())
        .collect())
}

fn parse_operand(
    operand: &str,
) -> Result<Box<dyn Fn(WorryLevel) -> WorryLevel>, Box<dyn std::error::Error>> {
    if operand == "old" {
        Ok(Box::new(|level: WorryLevel| level))
    } else if let Ok(value) = operand.parse::<WorryLevel>() {
        Ok(Box::new(move |_level: WorryLevel| value))
    } else {
        Err(Box::new(ParseError::new(format!(
            "Bad operand: {}",
            operand
        ))))
    }
}

fn parse_operator(
    operator: &str,
) -> Result<Box<dyn Fn(WorryLevel, WorryLevel) -> WorryLevel>, Box<dyn std::error::Error>> {
    match operator {
        "+" => Ok(Box::new(|a, b| a + b)),
        "-" => Ok(Box::new(|a, b| a - b)),
        "*" => Ok(Box::new(|a, b| a * b)),
        "/" => Ok(Box::new(|a, b| a / b)),
        _ => Err(Box::new(ParseError::new(format!(
            "Bad operator: {}",
            operator
        )))),
    }
}

fn parse_operation<R>(
    reader: &mut R,
) -> Result<Box<dyn Fn(WorryLevel) -> WorryLevel>, Box<dyn std::error::Error>>
where
    R: Iterator<Item = String>,
{
    lazy_static! {
        static ref OPERATION: Regex = Regex::new(r"^  Operation: new = (\S+) (\S) (\S+)$").unwrap();
    };

    let line = reader.next().unwrap();
    let capture = OPERATION.captures(line.as_str()).unwrap();

    let operand_a = capture.get(1).unwrap();
    let operand_b = capture.get(3).unwrap();
    let operator = capture.get(2).unwrap();

    let evaluator_a = parse_operand(operand_a.as_str())?;
    let evaluator_b = parse_operand(operand_b.as_str())?;
    let evaluator = parse_operator(operator.as_str())?;

    Ok(Box::new(move |value| {
        evaluator(evaluator_a(value), evaluator_b(value))
    }))
}

fn parse_condition<R>(
    reader: &mut R,
) -> Result<Box<dyn Fn(WorryLevel) -> bool>, Box<dyn std::error::Error>>
where
    R: Iterator<Item = String>,
{
    lazy_static! {
        static ref TEST: Regex = Regex::new(r"^  Test: divisible by (\d+)$").unwrap();
    };

    let line = reader.next().unwrap();
    let capture = TEST.captures(line.as_str()).unwrap();
    let divisor: usize = capture.get(1).unwrap().as_str().parse()?;

    Ok(Box::new(move |value| value % divisor == 0))
}

fn parse_branch<R>(reader: &mut R) -> Result<(bool, usize), Box<dyn std::error::Error>>
where
    R: Iterator<Item = String>,
{
    lazy_static! {
        static ref BRANCH: Regex =
            Regex::new(r"^    If (true|false): throw to monkey (\d+)$").unwrap();
    };

    let line = reader.next().unwrap();
    let capture = BRANCH.captures(line.as_str()).unwrap();
    let branch: bool = capture.get(1).unwrap().as_str().parse()?;
    let monkey: usize = capture.get(2).unwrap().as_str().parse()?;

    Ok((branch, monkey))
}

fn parse_test<R>(
    reader: &mut R,
) -> Result<Box<dyn Fn(WorryLevel) -> usize>, Box<dyn std::error::Error>>
where
    R: Iterator<Item = String>,
{
    lazy_static! {
        static ref IF_TRUE: Regex = Regex::new(r"^    If true: throw to monkey (\d+)$").unwrap();
        static ref IF_FALSE: Regex = Regex::new(r"^    If false: throw to monkey (\d+)$").unwrap();
    };

    let condition = parse_condition(reader)?;
    let branch_a = parse_branch(reader)?;
    let branch_b = parse_branch(reader)?;

    let monkeys = match (branch_a, branch_b) {
        ((true, monkey_a), (false, monkey_b)) => (monkey_a, monkey_b),
        ((false, monkey_a), (true, monkey_b)) => (monkey_b, monkey_a),
        _ => return Err(Box::new(ParseError::new(format!("missing branches")))),
    };

    Ok(Box::new(move |value| {
        if condition(value) {
            monkeys.0
        } else {
            monkeys.1
        }
    }))
}

fn parse_monkey<R>(reader: &mut R) -> Result<Option<Monkey>, Box<dyn std::error::Error>>
where
    R: Iterator<Item = String>,
{
    lazy_static! {
        static ref MONKEY: Regex = Regex::new(r"^Monkey (\d+):$").unwrap();
    };

    Ok(match reader.next() {
        Some(line) => {
            let cap = MONKEY.captures(line.as_str()).unwrap();
            let id: MonkeyId = cap.get(1).unwrap().as_str().parse().unwrap();
            let items = parse_starting_items(reader)?;
            let operation = parse_operation(reader)?;
            let test = parse_test(reader)?;
            Some(Monkey {
                id,
                items,
                operation,
                test,
            })
        }
        _ => None,
    })
}

fn parse_monkeys(filename: &str) -> Result<Vec<Monkey>, Box<dyn std::error::Error>> {
    let mut monkeys = vec![];

    let mut input = read_non_empty_lines(filename)?;
    while let Ok(Some(monkey)) = parse_monkey(&mut input) {
        monkeys.push(monkey);
    }

    for id in 0..monkeys.len() {
        assert_eq!(id, monkeys[id].id);
    }

    Ok(monkeys)
}

fn turn(monkeys: &mut Vec<Monkey>, counts: Option<&mut Vec<usize>>) {
    let mut item_inspections = vec![0; monkeys.len()];
    for id in 0..monkeys.len() {
        println!("Monkey {}:", id);
        while !monkeys[id].items.is_empty() {
            let items = monkeys[id].items.clone();
            monkeys[id].items.clear();
            for item in 0..items.len() {
                item_inspections[id] += 1;
                let worry_level = items[item];
                println!(
                    "  Monkey inspects an item with a worry level of {}.",
                    worry_level
                );
                let panic_level = monkeys[id].operation.as_ref()(worry_level);
                println!("    Worry level is adjusted to {}.", panic_level);
                let adjusted_level = panic_level / 3;
                println!(
                    "    Worry level is adjusted by boredom to {}.",
                    adjusted_level
                );
                let to_monkey_id = monkeys[id].test.as_ref()(adjusted_level);
                println!("    Item is passed on to monkey {}.", to_monkey_id);
                monkeys[to_monkey_id].items.push(adjusted_level);
            }
        }
    }
    counts.map(|v| {
        for i in 0..v.len() {
            v[i] += item_inspections[i];
        }
    });
}

fn monkey_business(counts: Vec<usize>) -> usize {
    let mut sorted_counts = counts.clone();
    sorted_counts.sort();
    let top_two = sorted_counts.into_iter().rev().take(2);
    return top_two.reduce(|a, b| a * b).unwrap();
}

fn main() {
    let mut monkeys = parse_monkeys("input.txt").unwrap();
    let mut counts = vec![0; monkeys.len()];
    for _ in 0..20 {
        turn(&mut monkeys, Some(&mut counts));
    }
    println!("monkey business: {}", monkey_business(counts));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_monkeys() {
        let monkeys = parse_monkeys("test_input.txt").unwrap();
        assert_eq!(monkeys.len(), 4);
    }

    #[test]
    fn result_after_one_turn() {
        let mut monkeys = parse_monkeys("test_input.txt").unwrap();
        turn(&mut monkeys, None);
        assert_eq!(monkeys[0].items, vec![20, 23, 27, 26]);
        assert_eq!(monkeys[1].items, vec![2080, 25, 167, 207, 401, 1046]);
        assert_eq!(monkeys[2].items, vec![]);
        assert_eq!(monkeys[3].items, vec![]);
    }

    #[test]
    fn result_after_twenty_turns() {
        let mut monkeys = parse_monkeys("test_input.txt").unwrap();
        for _ in 0..20 {
            turn(&mut monkeys, None);
        }
        assert_eq!(monkeys[0].items, vec![10, 12, 14, 26, 34]);
        assert_eq!(monkeys[1].items, vec![245, 93, 53, 199, 115]);
        assert_eq!(monkeys[2].items, vec![]);
        assert_eq!(monkeys[3].items, vec![]);
    }

    #[test]
    fn inspection_counts() {
        let mut monkeys = parse_monkeys("test_input.txt").unwrap();
        let mut counts = vec![0; monkeys.len()];
        for _ in 0..20 {
            turn(&mut monkeys, Some(&mut counts));
        }
        assert_eq!(counts, [101, 95, 7, 105]);
        assert_eq!(monkey_business(counts), 10605);
    }
}
