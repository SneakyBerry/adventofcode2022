use aoc_runner_derive::aoc_main;
aoc_main! { lib = adventofcode2022 }

#[cfg(test)]
mod tests {
    use adventofcode2022::solutions::{day1, day2, day3, day4, day5};

    #[test]
    fn test_day1() {
        let input = std::fs::read_to_string("./input/2022/day1.txt").unwrap();
        assert_eq!(day1::solve1(&input), 69883);
        assert_eq!(day1::solve2(&input), 207576);
        let input = std::fs::read_to_string("./input/2022/day2.txt").unwrap();
        assert_eq!(day2::solve1(&input), 10816);
        assert_eq!(day2::solve2(&input), 11657);
        let input = std::fs::read_to_string("./input/2022/day3.txt").unwrap();
        assert_eq!(day3::solve1(&input), 8298);
        assert_eq!(day3::solve2(&input), 2708);
        let input = std::fs::read_to_string("./input/2022/day4.txt").unwrap();
        assert_eq!(day4::solve1(&input), 524);
        assert_eq!(day4::solve2(&input), 798);
        let input = std::fs::read_to_string("./input/2022/day5.txt").unwrap();
        assert_eq!(day5::solve1(&input), "HBTMTBSDC");
        assert_eq!(day5::solve2(&input), "PQTJRSHWS");
    }
}
