pub mod solutions;

use aoc_runner_derive::aoc_lib;

#[macro_use]
extern crate aoc_runner_derive;

aoc_lib! { year = 2022 }

#[macro_export]
macro_rules! aoc_tests {
    (
        $ (
            name: $name: ident;
            input: $input: expr;
            task1: $task1_test: expr;
            task2: $task2_test: expr;
        ) *
    ) => {
        #[cfg(test)]
        mod tests {
            use super::*;

        $ (
            #[test]
            fn $name() {
                    let input = $input;
                    assert_eq!(solve1(&input), $task1_test);
                    assert_eq!(solve2(&input), $task2_test);
            }
        ) *
        }
    };
}
