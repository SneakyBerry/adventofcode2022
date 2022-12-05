fn solve(input: &str, compare_fn: fn((usize, usize), (usize, usize)) -> bool) -> usize {
    input
        .lines()
        .map(|line| {
            let split = line.split(",").collect::<Vec<_>>();
            let left_elv = split[0]
                .split("-")
                .map(|x| str::parse::<usize>(x).unwrap())
                .collect::<Vec<_>>();
            let right_elv = split[1]
                .split("-")
                .map(|x| str::parse::<usize>(x).unwrap())
                .collect::<Vec<_>>();
            compare_fn((left_elv[0], left_elv[1]), (right_elv[0], right_elv[1])) as usize
        })
        .sum()
}

#[aoc(day4, part1)]
pub fn solve1(input: &str) -> usize {
    solve(input, |(ll, lr), (rl, rr)| {
        ll <= rl && lr >= rr || ll >= rl && lr <= rr
    })
}

#[aoc(day4, part2)]
pub fn solve2(input: &str) -> usize {
    solve(input, |(ll, lr), (rl, rr)| {
        ll >= rl && ll <= rr || lr >= rl && lr <= rr || rr >= ll && rr <= lr || rr >= ll && rr <= lr
    })
}
