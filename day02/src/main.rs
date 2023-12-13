use aoc2022::ParseError;

#[derive(Copy, Clone, PartialEq)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl Hand {
    fn loser_again(hand: Self) -> Self {
        match hand {
            Self::Rock => Self::Scissors,
            Self::Paper => Self::Rock,
            Self::Scissors => Self::Paper,
        }
    }

    fn winner_again(hand: Self) -> Self {
        match hand {
            Self::Rock => Self::Paper,
            Self::Paper => Self::Scissors,
            Self::Scissors => Self::Rock,
        }
    }

    fn points(self) -> usize {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }

    fn parse_opponent_hand(value: &str) -> Result<Self, ParseError> {
        match value {
            "A" => Ok(Self::Rock),
            "B" => Ok(Self::Paper),
            "C" => Ok(Self::Scissors),
            _ => Err(ParseError::new(format!("invalid opponent hand: {}", value))),
        }
    }
}

fn parse_part1_strategy(strategy: &str, _opponent_hand: Hand) -> Result<Hand, ParseError> {
    match strategy {
        "X" => Ok(Hand::Rock),
        "Y" => Ok(Hand::Paper),
        "Z" => Ok(Hand::Scissors),
        _ => Err(ParseError::new(format!("invalid strategy: {}", strategy))),
    }
}

fn parse_part2_strategy(strategy: &str, opponent_hand: Hand) -> Result<Hand, ParseError> {
    match strategy {
        "X" => Ok(Hand::loser_again(opponent_hand)),
        "Y" => Ok(opponent_hand),
        "Z" => Ok(Hand::winner_again(opponent_hand)),
        _ => Err(ParseError::new(format!("invalid strategy: {}", strategy))),
    }
}

fn process_round<F>(round: &str, strategy: F) -> Result<usize, Box<dyn std::error::Error>>
where
    F: Fn(&str, Hand) -> Result<Hand, ParseError>,
{
    match round.split_once(' ') {
        Some((opponent, strategy_name)) => {
            let opponent_hand = Hand::parse_opponent_hand(opponent)?;
            let player_hand = strategy(strategy_name, opponent_hand)?;
            let mut score = player_hand.points();
            if Hand::loser_again(player_hand) == opponent_hand {
                score += 6
            } else if player_hand == opponent_hand {
                score += 3
            }
            Ok(score)
        }
        _ => Ok(0),
    }
}

fn process_rounds<F>(filename: &str, strategy: F) -> Result<usize, Box<dyn std::error::Error>>
where
    F: Fn(&str, Hand) -> Result<Hand, ParseError>,
{
    let mut score: usize = 0;
    for line in aoc2022::read_lines(filename)? {
        score += process_round(&line?, &strategy)?;
    }

    Ok(score)
}

fn main() {
    let part1_score = process_rounds("input.txt", parse_part1_strategy).unwrap();
    println!("part 1: {}", part1_score);

    let part2_score = process_rounds("input.txt", parse_part2_strategy).unwrap();
    println!("part 2: {}", part2_score);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_strategy_rock_vs_scissors() {
        assert_eq!(process_round("A Y", parse_part1_strategy).unwrap(), 8);
    }

    #[test]
    fn part1_strategy_scissors_vs_rock() {
        assert_eq!(process_round("B X", parse_part1_strategy).unwrap(), 1);
    }

    #[test]
    fn part1_strategy_paper_vs_paper() {
        assert_eq!(process_round("C Z", parse_part1_strategy).unwrap(), 6);
    }

    #[test]
    fn part2_strategy_rock_vs_scissors() {
        assert_eq!(process_round("A Y", parse_part2_strategy).unwrap(), 4);
    }

    #[test]
    fn part2_strategy_scissors_vs_rock() {
        assert_eq!(process_round("B X", parse_part2_strategy).unwrap(), 1);
    }

    #[test]
    fn part2_strategy_paper_vs_paper() {
        assert_eq!(process_round("C Z", parse_part2_strategy).unwrap(), 7);
    }
}
