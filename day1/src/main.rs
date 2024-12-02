use day1::LocationIds;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let location_ids: LocationIds<u64> = INPUT.parse().expect("Input must parse");
    let differences = location_ids.get_differences();
    println!("Day 1 Part 1: {}", differences.into_iter().sum::<u64>());

    let similarities = location_ids.get_similarities();
    println!("Day 1 Part 2: {}", similarities.into_iter().sum::<u64>());
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = include_str!("test_input.txt");
    #[test]
    fn test_differences() {
        let location_ids: LocationIds<u8> = INPUT.parse().unwrap();
        let differences = location_ids.get_differences();
        assert_eq!(differences, vec![2, 1, 0, 1, 2, 5])
    }
    #[test]
    fn test_similarities() {
        let location_ids: LocationIds<u8> = INPUT.parse().unwrap();
        let similarities = location_ids.get_similarities();
        assert_eq!(similarities, vec![9, 4, 0, 0, 9, 9])
    }
}
