use crate::aoc_tests;

type Crates = Vec<Vec<char>>;
type Instructions = Vec<(usize, usize, usize)>;

pub fn parse_input(input: &str) -> (Crates, Instructions) {
    let input = input.replace("\r\n", "\n");
    let input = input.split("\n\n").collect::<Vec<_>>();
    let crates = input[0].lines().collect::<Vec<_>>();
    let instructions = input[1].lines().collect::<Vec<_>>();
    let columns_count = crates[crates.len() - 1]
        .split_whitespace()
        .map(|x| str::parse::<usize>(x).unwrap())
        .max()
        .unwrap();
    let rows_count = crates.len();
    let mut state = vec![vec![]; columns_count];
    for (j, col) in state.iter_mut().enumerate().take(columns_count) {
        for row in crates[..rows_count - 1].iter() {
            let char = row.get(j * 4..j * 4 + 3).unwrap_or("").trim();
            if char.is_empty() {
                continue;
            } else {
                col.push(char.chars().nth(1).expect("It can't be None"));
            }
        }
    }
    let mut parsed_instruction = vec![];
    for instruction in instructions {
        let mut instruction = instruction.split_whitespace();
        let count = get_next(&mut instruction);
        let from = get_next(&mut instruction);
        let to = get_next(&mut instruction);
        parsed_instruction.push((count, from, to));
    }
    (state, parsed_instruction)
}

fn get_next<'lt, T>(instruction: &mut T) -> usize
where
    T: Iterator<Item = &'lt str>,
{
    instruction.nth(1).unwrap().parse::<usize>().unwrap()
}

#[aoc(day5, part1)]
pub fn solve1(input: &str) -> String {
    let (mut state, instructions) = parse_input(input);
    for (count, from, to) in &instructions {
        for _ in 0..*count {
            let x = state[from - 1].remove(0);
            state[to - 1].insert(0, x);
        }
    }
    state.iter_mut().filter_map(|col| col.first()).collect()
}

#[aoc(day5, part2)]
pub fn solve2(input: &str) -> String {
    let (mut state, instructions) = parse_input(input);
    for (count, from, to) in &instructions {
        for i in 0..*count {
            let x = state[from - 1].remove(0);
            state[to - 1].insert(i, x);
        }
    }
    state.iter_mut().filter_map(|col| col.first()).collect()
}

aoc_tests!(
    name: day5_test1;
    input: "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
    task1: "CMZ";
    task2: "MCD";
);
