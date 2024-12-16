use day6::{has_loop, parser, Guard, Position};
use std::collections::HashSet;

const INPUT: &str = include_str!("input.txt");

fn solve_part_1(guard: &mut Guard) -> usize {
    let mut visited: HashSet<Position> = HashSet::from([guard.position]);
    visited.extend(guard.map(|step| step.0));
    visited.len()
}

fn solve_part_2(guard: &mut Guard) -> usize {
    let original_position = guard.position;
    let mut visited: HashSet<Position> = HashSet::from([original_position]);
    visited.extend(guard.clone().map(|step| step.0));
    let new_obstacles = visited
        .into_iter()
        .filter(|&location| {
            if location == original_position { return false; }
            let mut new_guard = guard.clone();
            new_guard.add_obstacle(location);
            has_loop(new_guard)
        })
        .collect::<Vec<_>>();
    new_obstacles.len()
}

fn main() -> anyhow::Result<()> {
    let guard = parser::parse(INPUT)?;
    let part1 = solve_part_1(&mut guard.clone());
    println!("Day 6 Part 1: {}", part1);
    let part2 = solve_part_2(&mut guard.clone());
    println!("Day 6 Part 2: {}", part2);
    Ok(())
}

#[cfg(test)]
mod tests {
    use day6::parser;

    const INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn solve_part_1() {
        let mut guard = parser::parse(INPUT).unwrap();
        let part1 = super::solve_part_1(&mut guard);
        assert_eq!(part1, 41);
    }
    #[test]
    fn solve_part_2() {
        let mut guard = parser::parse(INPUT).unwrap();
        let part1 = super::solve_part_2(&mut guard);
        assert_eq!(part1, 6);
    }
}
