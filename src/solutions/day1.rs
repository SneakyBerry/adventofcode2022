use crate::aoc_tests;

fn solve(input: &str, top_size: usize) -> usize {
    let mut calories = input.lines().fold(vec![0], |mut acc, x| {
        if let Ok(cal) = str::parse::<usize>(x) {
            *acc.last_mut().expect("It can't be empty") += cal
        } else {
            acc.push(0)
        };
        acc
    });
    calories.sort();
    calories.iter().rev().take(top_size).sum()
}

#[aoc(day1, part1)]
pub fn solve1(input: &str) -> usize {
    solve(input, 1)
}

#[aoc(day1, part2)]
pub fn solve2(input: &str) -> usize {
    solve(input, 3)
}

aoc_tests!(
    name: day1_test1;
    input: "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
    task1: 24000;
    task2: 45000;
);
