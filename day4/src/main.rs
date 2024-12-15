use day4::{filter_xmas, filter_x_mas, parser};
use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");

fn solve_part1(s: &str) -> usize {
    let mut letters = HashMap::new();
    let mut idxs = Vec::new();

    for (idx, letter) in parser::parse(s) {
        // dbg!(idx, letter);
        idxs.push(idx);
        letters.insert(idx, letter);
    }
    idxs.iter()
        .filter_map(|loc| {
            let filtered = filter_xmas(*loc, &letters);
            if filtered.is_empty() {
                None
            } else {
                Some(filtered.len() / 4) // count how many XMASes start from this square
            }
        })
        .sum()
}

fn solve_part2(s: &str) -> usize {
    let letters: HashMap<_, _> = parser::parse(s).into_iter().collect();
    letters.keys().filter_map(|loc| {
        let filtered = filter_x_mas(*loc, &letters);
        if filtered.is_empty() { None } else {
            Some(filtered.len() / 5)
        }
    }).sum()
}

fn main() {
    println!("Day 4 Part 1: {}", solve_part1(INPUT));
    println!("Day 4 Part 2: {}", solve_part2(INPUT));
}

#[cfg(test)]
mod main_tests {
    use super::*;

    const INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn test_solve_part_1() {
        assert_eq!(solve_part1(INPUT), 18usize)
    }
    #[test]
    fn test_solve_part_2() {
        assert_eq!(solve_part2(INPUT), 9usize)
    }
}
