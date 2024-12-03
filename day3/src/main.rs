const INPUT: &str = include_str!("input.txt");

fn solve(muls: Vec<day3::Mul>) -> u64 {
    muls.into_iter().map(std::convert::Into::<u64>::into).sum()
}

fn main() {
    let parsed: Vec<day3::Mul> = day3::parser::parse(INPUT, false).expect("Input must parse");
    let part1 = solve(parsed);
    let parsed2: Vec<day3::Mul> = day3::parser::parse(INPUT, true).expect("Input must parse");
    let part2 = solve(parsed2);
    println!("Day3 Part1: {}", part1);
    println!("Day3 Part2: {}", part2);
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn test_solve_part_1() {
        let parsed: Vec<day3::Mul> = day3::parser::parse(INPUT, false).unwrap();
        assert_eq!(solve(parsed), 161)
    }
    #[test]
    fn test_solve_part_2() {
        let parsed: Vec<day3::Mul> = day3::parser::parse(INPUT, true).unwrap();
        assert_eq!(solve(parsed), 48)
    }
}
