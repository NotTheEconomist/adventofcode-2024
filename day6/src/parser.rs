use std::collections::HashSet;

use crate::{errors, Facing, Guard, Position};

enum ParsedPositional {
    Guard(Position, Facing),
    Obstacle(Position),
    Error(errors::Day6Error),
}

pub fn parse(input: &str) -> Result<Guard, errors::Day6Error> {
    let (position, facing, obstacles, max_bounds): (
        Option<Position>,
        Option<Facing>,
        HashSet<Position>,
        Position,
    ) = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, s)| match s {
                '.' => None,
                '#' => Some(ParsedPositional::Obstacle((x, y))),
                '^' => Some(ParsedPositional::Guard((x, y), Facing::North)),
                'v' => Some(ParsedPositional::Guard((x, y), Facing::South)),
                '<' => Some(ParsedPositional::Guard((x, y), Facing::West)),
                '>' => Some(ParsedPositional::Guard((x, y), Facing::East)),
                otherwise => Some(ParsedPositional::Error(errors::Day6Error::ParseError(
                    input.to_string(),
                    format!("Expected one of \". # & v < >\", but got {}", otherwise),
                ))),
            })
        })
        .try_fold(
            (
                None::<Position>,
                None::<Facing>,
                HashSet::<Position>::new(),
                (usize::MIN, usize::MIN),
            ),
            |(mut guard_pos, mut guard_facing, mut obstacles, (mut max_x, mut max_y)),
             positional| {
                match positional {
                    ParsedPositional::Guard(position @ (x, y), facing)
                        if guard_pos.is_none() && guard_facing.is_none() =>
                    {
                        if x > max_x {
                            max_x = x;
                        }
                        if y > max_y {
                            max_y = y;
                        }
                        guard_pos = Some(position);
                        guard_facing = Some(facing);
                    }
                    ParsedPositional::Guard(_, _) => return Err(errors::Day6Error::ParseError(input.to_string(), "Two guards are included in input string.".to_string())),
                    ParsedPositional::Obstacle(position@(x, y)) => {
                        if x > max_x {
                            max_x = x;
                        }
                        if y > max_y {
                            max_y = y;
                        }
                        obstacles.insert(position);
                    },
                    ParsedPositional::Error(day6_error) => return Err(day6_error),
                }
                Ok((guard_pos, guard_facing, obstacles, (max_x, max_y)))
            },
        )?;
    let position = position.ok_or(errors::Day6Error::ParseError(input.to_string(), "Guard position was not found".to_string()))?;
    let facing = facing.ok_or(errors::Day6Error::ParseError(input.to_string(), "Guard facing was not found".to_string()))?;
    Ok(Guard { position, facing, obstacles, bounds: ((0, 0), max_bounds) })
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    const INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn obstacle_parse() {
        let mut expected = HashSet::new();
        expected.insert((4, 0));
        expected.insert((9, 1));
        expected.insert((2, 3));
        expected.insert((7, 4));
        expected.insert((1, 6));
        expected.insert((8, 7));
        expected.insert((0, 8));
        expected.insert((6, 9));
        let guard: super::Guard = super::parse(INPUT).expect("Guard must parse in test cases");
        assert_eq!(guard.obstacles, expected);
    }

    #[test]
    fn guard_position_and_facing_parse() {
        let expected_position = (4, 6);
        let expected_facing = super::Facing::North;
        let guard: super::Guard = super::parse(INPUT).expect("Guard must parse in test cases");
        assert_eq!(guard.position, expected_position);
        assert_eq!(guard.facing, expected_facing);
    }
}
