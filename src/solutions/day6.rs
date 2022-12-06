use crate::aoc_tests;

fn solution(input: &str, distinct_num: usize) -> Option<usize> {
    for i in 0..input.len() {
        let mut buf = 0_u32;
        let chars = &input[i..i + distinct_num];
        for c in chars.chars() {
            buf |= 1 << (c as u8 - b'a');
        }
        if buf.count_ones() == distinct_num as u32 {
            return Some(i + distinct_num);
        }
    }
    None
}

#[aoc(day6, part1)]
pub fn solve1(input: &str) -> Option<usize> {
    solution(input, 4)
}

#[aoc(day6, part2)]
pub fn solve2(input: &str) -> Option<usize> {
    solution(input, 14)
}

aoc_tests!(
    name: day6_test1;
    input: "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    task1: Some(7);
    task2: Some(19);

    name: day6_test2;
    input: "bvwbjplbgvbhsrlpgdmjqwftvncz";
    task1: Some(5);
    task2: Some(23);

    name: day6_test3;
    input: "nppdvjthqldpwncqszvftbrmjlhg";
    task1: Some(6);
    task2: Some(23);

    name: day6_test4;
    input: "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    task1: Some(10);
    task2: Some(29);

    name: day6_test5;
    input: "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
    task1: Some(11);
    task2: Some(26);
);
