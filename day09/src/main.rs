use std::{
    collections::HashSet,
    fmt,
    ops::{Add, AddAssign, Mul, Sub},
};

use aoc2022::{read_lines, ParseError};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    const ZERO: Self = Self { x: 0, y: 0 };
    const UP: Self = Self { x: 0, y: 1 };
    const DOWN: Self = Self { x: 0, y: -1 };
    const LEFT: Self = Self { x: -1, y: 0 };
    const RIGHT: Self = Self { x: 1, y: 0 };
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}", self.x, self.y)
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, rhs: Position) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Position {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Position {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<i32> for Position {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

fn simulate_moves(filename: &str, knot_count: usize) -> Result<usize, Box<dyn std::error::Error>> {
    assert_ne!(knot_count, 0);

    let mut knots = vec![Position::ZERO; knot_count];

    let mut visited_positions = HashSet::new();
    visited_positions.insert(*knots.last().unwrap());

    for line in read_lines(filename)? {
        if let Some((direction, count)) = line?.split_once(" ") {
            let head_movement = match direction {
                "U" => Ok(Position::UP),
                "D" => Ok(Position::DOWN),
                "L" => Ok(Position::LEFT),
                "R" => Ok(Position::RIGHT),
                _ => Err(ParseError::new(format!("invalid direction: {}", direction))),
            }?;
            let times: usize = count.parse()?;

            for _ in 0..times {
                *knots.first_mut().unwrap() += head_movement;

                for i in 0..knot_count - 1 {
                    let head = knots[i];
                    let tail = &mut knots[i + 1];

                    let delta = head - *tail;
                    let tail_movement = if delta.x.abs() > 1 && delta.y.abs() > 1 {
                        Position {
                            x: delta.x.signum(),
                            y: delta.y.signum(),
                        }
                    } else if delta.x.abs() > 1 {
                        Position {
                            x: delta.x.signum(),
                            y: delta.y,
                        }
                    } else if delta.y.abs() > 1 {
                        Position {
                            x: delta.x,
                            y: delta.y.signum(),
                        }
                    } else {
                        Position::ZERO
                    };

                    *tail = *tail + tail_movement;
                }

                visited_positions.insert(*knots.last().unwrap());
            }
        }
    }

    Ok(visited_positions.len())
}

fn main() {
    println!("part 1: {}", simulate_moves("input.txt", 2).unwrap());

    println!("part 2: {}", simulate_moves("input.txt", 10).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_knot() {
        assert_eq!(simulate_moves("input_test.txt", 2).unwrap(), 13);
    }

    #[test]
    fn many_knots() {
        assert_eq!(simulate_moves("input_test.txt", 10).unwrap(), 1);
        assert_eq!(simulate_moves("input_test2.txt", 10).unwrap(), 36);
    }
}
