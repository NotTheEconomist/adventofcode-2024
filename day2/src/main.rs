#![feature(array_windows)]

use day2::{is_safe, is_safe_with_problem_dampener};

const INPUT: &str = include_str!("input.txt");

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .trim()
        .lines()
        .map(|line| line.split_whitespace().flat_map(str::parse).collect())
        .collect()
}

#[allow(dead_code)]
/// This was useful in figuring out which level was failing to show as safe
fn debug_levels<F>(levels: &[Vec<u8>], safety_fn: F)
where
    F: Fn(&[u8]) -> bool,
{
    for level in levels.iter() {
        let values = level
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(" ");
        let diffs = level
            .array_windows()
            .map(|[prev, next]| (*next as i8 - *prev as i8).to_string())
            .collect::<Vec<String>>()
            .join(" ");
        let is_safe = safety_fn(level);
        println!(
            "{} {} || {}",
            if is_safe { "    SAFE" } else { "NOT SAFE" },
            values,
            diffs
        );
    }
}

fn solve_part_1(input: &str) -> usize {
    let parsed_input = parse_input(input);
    let safe_levels = parsed_input
        .into_iter()
        .filter(|line| is_safe(line))
        .collect::<Vec<_>>();

    safe_levels.len()
}

fn solve_part_2(input: &str) -> usize {
    let parsed_input = parse_input(input);
    let safe_levels = parsed_input
        .into_iter()
        .filter(|line| is_safe_with_problem_dampener(line))
        .collect::<Vec<_>>();

    safe_levels.len()
}

fn main() {
    let result_1 = solve_part_1(INPUT);
    let result_2 = solve_part_2(INPUT);
    println!("Day 2 Part 1: {}", result_1);
    println!("Day 2 Part 2: {}", result_2);
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("test_input.txt");

    use super::*;

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part_1(INPUT), 2)
    }
    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part_2(INPUT), 4)
    }
}
