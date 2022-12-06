use std::cmp::Ordering;
use crate::aoc_tests;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Figure {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Figure {
    fn get_winner(&self) -> Self {
        match self {
            Figure::Rock => Figure::Paper,
            Figure::Paper => Figure::Scissors,
            Figure::Scissors => Figure::Rock,
        }
    }

    fn get_looser(&self) -> Self {
        match self {
            Figure::Rock => Figure::Scissors,
            Figure::Paper => Figure::Rock,
            Figure::Scissors => Figure::Paper,
        }
    }
}

impl TryFrom<char> for Figure {
    type Error = std::io::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' | 'X' => Ok(Figure::Rock),
            'B' | 'Y' => Ok(Figure::Paper),
            'C' | 'Z' => Ok(Figure::Scissors),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Invalid figure: {}", value),
            )),
        }
    }
}

impl PartialOrd for Figure {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if &self.get_winner() == other {
            Some(Ordering::Less)
        } else if &self.get_looser() == other {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Equal)
        }
    }
}


#[aoc(day2, part1)]
pub fn solve1(input: &str) -> usize {
    input.lines().fold(0, |mut score, line| {
        let mut parsed = line.chars().map(Figure::try_from);
        match (parsed.next(), parsed.last()) {
            (Some(Ok(left)), Some(Ok(right))) => {
                score += right as usize;
                if left == right {
                    score += 3;
                } else if left < right {
                    score += 6;
                }
            }
            line => panic!("Invalid input: {:?}{:?}", line.0, line.1),
        };
        score
    })
}

#[aoc(day2, part2)]
pub fn solve2(input: &str) -> usize {
    input.lines().fold(0, |mut score, line| {
        match line.as_bytes() {
            [left, .., right] => {
                let left = Figure::try_from(*left as char).unwrap();
                match right {
                    b'X' => score += left.get_looser() as usize,
                    b'Y' => score += 3 + left as usize,
                    b'Z' => score += 6 + left.get_winner() as usize,
                    _ => panic!("Invalid input: {:?}", line),
                }
            }
            line => panic!("Invalid input: {:?}", line),
        };
        score
    })
}

aoc_tests!(
    name: day2_test1;
    input: "A Y
B X
C Z";
    task1: 15;
    task2: 12;
);
