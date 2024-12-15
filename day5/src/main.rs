use std::iter::Sum;
use std::{ops::Add, str::FromStr};
use std::hash::Hash;

use day5::{parser, reorder_update_list, Input};

const INPUT: &str = include_str!("input.txt");


fn main() {
    let input: Input<u64> = parser::parse(INPUT).expect("Given input must parse");
    let part1 = solve_part_1(&input);
    let part2 = solve_part_2(&input);

    println!("Day 5 Part 1: {}", part1);
    println!("Day 5 Part 2: {}", part2);
}

fn solve_part_1<T>(input: &Input<T>) -> T where T: FromStr+Add+Hash+Eq+Ord+Copy, T: Sum {
    input.valid_update_lists().into_iter().map(|list| {
        let midpoint: usize = (list.len() - 1) / 2;
        *list.get(midpoint).expect("All lists must have a midpoint")
    }).sum()
}
fn solve_part_2<T>(input: &Input<T>) -> T where T: std::fmt::Debug+FromStr+Add+Hash+Eq+Ord+Copy, T: Sum {
    let rules = &input.ordering_rules;
    input.invalid_update_lists().into_iter().map(|list|
        reorder_update_list(list, rules).unwrap()
    ).map(|list| {
        let midpoint: usize = (list.len() - 1) / 2;
        *list.get(midpoint).expect("All lists must have a midpoint")
        }).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn solve_part_1() {
        let input: Input<u8> = parser::parse(INPUT).unwrap();
        let solution = crate::solve_part_1(&input);
        assert_eq!(solution, 143)
    }
    #[test]
    fn solve_part_2() {
        let input: Input<u8> = parser::parse(INPUT).unwrap();
        let solution = crate::solve_part_2(&input);
        assert_eq!(solution, 123)
    }
}
