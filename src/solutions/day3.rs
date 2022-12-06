use crate::aoc_tests;
use std::ops::BitOr;

fn get_priority(x: char) -> usize {
    let x = x as u8;
    if (b'a'..=b'z').contains(&x) {
        1 << (x - 96)
    } else if(b'A'..=b'Z').contains(&x) {
        1 << (x - 38)
    } else {
        panic!("Invalid input: {:?}", x as char)
    }
}

fn get_priorities(input: &str) -> usize {
    input
        .chars()
        .map(get_priority)
        .reduce(BitOr::bitor)
        .unwrap_or_default()
}

#[aoc(day3, part1)]
pub fn solve1(input: &str) -> usize {
    input.lines().fold(0, |mut priorities, rucksack| {
        let (left, right) = rucksack.split_at(rucksack.len() / 2);
        let left = get_priorities(left);
        let right = get_priorities(right);
        let intersection = left & right;
        for i in 0..64 {
            if intersection & (1 << i) != 0 {
                priorities += i;
            }
        }
        priorities
    })
}

#[aoc(day3, part2)]
pub fn solve2(input: &str) -> usize {
    input
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .fold(0, |mut priorities, rucksack| {
            let mut intersection = usize::MAX;
            for line in rucksack {
                intersection &= get_priorities(line);
            }
            for i in 0..64 {
                if intersection & (1 << i) != 0 {
                    priorities += i;
                }
            }
            priorities
        })
}

aoc_tests!(
    name: day3_test1;
    input: "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
    task1: 157;
    task2: 70;
);
