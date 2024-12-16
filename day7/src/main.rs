use day7::{parser, Equation, Operator, OperatorPart2};

const INPUT: &str = include_str!("input.txt");

fn solve_part_1(equations: Vec<Equation<Operator>>) -> i64 {
    equations.into_iter().filter_map(|mut eq| {
        if eq.solve().is_some() {
            Some(eq.result)
        } else {
            None
        }
    }).sum()
}

fn solve_part_2(equations: Vec<Equation<OperatorPart2>>) -> i64 {
    equations.into_iter().filter_map(|mut eq| {
        if eq.solve().is_some() {
            Some(eq.result)
        } else {
            None
        }
    }).sum()
}

fn main() {
    let equations = parser::parse(INPUT).expect("Input must parse");
    let part1 = solve_part_1(equations);
    let equations = parser::parse(INPUT).expect("Input must parse");
    let part2 = solve_part_2(equations);
    println!("Day 7 Part 1: {}", part1);
    println!("Day 7 Part 2: {}", part2);
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn solve_part_1() {
        let eqs = day7::parser::parse(INPUT).unwrap();
        assert_eq!(super::solve_part_1(eqs), 3749);
    }
    #[test]
    fn solve_part_2() {
        let eqs = day7::parser::parse(INPUT).unwrap();
        assert_eq!(super::solve_part_2(eqs), 11387);
    }
}

