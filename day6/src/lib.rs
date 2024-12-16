use std::{collections::HashSet, fmt::Display, fmt::Write};

pub mod errors;
pub mod parser;

pub type Position = (usize, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, std::hash::Hash)]
pub enum Facing {
    North,
    East,
    South,
    West,
}

impl Facing {
    pub fn turn_right(&self) -> Self {
        match self {
            Facing::North => Facing::East,
            Facing::East => Facing::South,
            Facing::South => Facing::West,
            Facing::West => Facing::North,
        }
    }

    pub fn turn_left(&self) -> Self {
        self.turn_right().turn_right().turn_right()
    }

    pub fn turn_around(&self) -> Self {
        self.turn_right().turn_right()
    }

    pub fn step_from(&self, position: &Position) -> Option<Position> {
        match self {
            Facing::North => Some((position.0, position.1.checked_sub(1)?)),
            Facing::East => Some((position.0.checked_add(1)?, position.1)),
            Facing::South => Some((position.0, position.1.checked_add(1)?)),
            Facing::West => Some((position.0.checked_sub(1)?, position.1)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Guard {
    pub position: Position,
    pub(crate) facing: Facing,
    pub obstacles: HashSet<Position>,
    pub(crate) bounds: (Position, Position),
}

impl Guard {
    pub fn add_obstacle(&mut self, obstacle_position: Position) {
        self.obstacles.insert(obstacle_position);
    }
}

impl Display for Guard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..=self.bounds.1 .1 {
            for col in 0..=self.bounds.1 .0 {
                let icon = if (col, row) == self.position {
                    match self.facing {
                        Facing::North => '^',
                        Facing::East => '>',
                        Facing::South => 'v',
                        Facing::West => '<',
                    }
                } else if self.obstacles.contains(&(col, row)) {
                    '#'
                } else {
                    '.'
                };
                f.write_char(icon)?
            }
            f.write_char('\n')?
        }
        Ok(())
    }
}

impl Iterator for Guard {
    type Item = (Position, Facing);

    fn next(&mut self) -> Option<Self::Item> {
        let mut i = 0;
        loop {
            i += 1;
            // In practice, this will catch falling off the top or the left
            let next_pos = self.facing.step_from(&self.position)?;

            // bounds check
            if next_pos.0 < self.bounds.0 .0
                || next_pos.0 > self.bounds.1 .0
                || next_pos.1 < self.bounds.0 .1
                || next_pos.1 > self.bounds.1 .1
            {
                break None;
            }

            if self.obstacles.contains(&next_pos) {
                if i < 4 {
                    // we're still trying to find the right facing
                    self.facing = self.facing.turn_right();
                    continue;
                } else {
                    // we've spun completely in a circle, let's abort
                    break None;
                }
            } else {
                self.position = next_pos;
                break Some((next_pos, self.facing));
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct GuardWatcher {
    // Watches a guard's patrol to see if it's in a loop
    // at every step, if a guard has passed that position facing in the same direction, it is stuck
    // in a loop
    guard: Guard,
    visited: HashSet<(Position, Facing)>,
}

impl GuardWatcher {
    fn has_loop(mut self) -> bool {
        for (step, facing) in self.guard {
            if self.visited.contains(&(step, facing)) {
                return true;
            } else {
                self.visited.insert((step, facing));
            }
        }
        false
    }

    fn new(guard: Guard) -> Self {
        Self {
            guard,
            visited: HashSet::new(),
        }
    }
}

pub fn has_loop(guard: Guard) -> bool {
    GuardWatcher::new(guard).has_loop()
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn test_guard_walk() {
        let mut guard = super::parser::parse(INPUT).expect("Guard must parse in test cases");
        assert_eq!(guard.next().map(|x| x.0), Some((4, 5)));
        assert_eq!(guard.next().map(|x| x.0), Some((4, 4)));
        assert_eq!(guard.next().map(|x| x.0), Some((4, 3)));
        assert_eq!(guard.next().map(|x| x.0), Some((4, 2)));
        assert_eq!(guard.next().map(|x| x.0), Some((4, 1)));
        assert_eq!(guard.next().map(|x| x.0), Some((5, 1)));
        assert_eq!(guard.next().map(|x| x.0), Some((6, 1)));
        assert_eq!(guard.next().map(|x| x.0), Some((7, 1)));
        assert_eq!(guard.next().map(|x| x.0), Some((8, 1)));
        assert_eq!(guard.next().map(|x| x.0), Some((8, 2)));
        assert_eq!(guard.next().map(|x| x.0), Some((8, 3)));
        assert_eq!(guard.next().map(|x| x.0), Some((8, 4)));
        assert_eq!(guard.next().map(|x| x.0), Some((8, 5)));
        assert_eq!(guard.next().map(|x| x.0), Some((8, 6)));
        assert_eq!(guard.next().map(|x| x.0), Some((7, 6)));
        assert_eq!(guard.next().map(|x| x.0), Some((6, 6)));
        assert_eq!(guard.next().map(|x| x.0), Some((5, 6)));
        assert_eq!(guard.next().map(|x| x.0), Some((4, 6)));
        assert_eq!(guard.next().map(|x| x.0), Some((3, 6)));
        assert_eq!(guard.next().map(|x| x.0), Some((2, 6)));
        assert_eq!(guard.next().map(|x| x.0), Some((2, 5)));
        assert_eq!(guard.next().map(|x| x.0), Some((2, 4)));
        assert_eq!(guard.next().map(|x| x.0), Some((3, 4)));
        assert_eq!(guard.next().map(|x| x.0), Some((4, 4)));
        assert_eq!(guard.next().map(|x| x.0), Some((5, 4)));
        assert_eq!(guard.next().map(|x| x.0), Some((6, 4)));
        assert_eq!(guard.next().map(|x| x.0), Some((6, 5)));
        assert_eq!(guard.next().map(|x| x.0), Some((6, 6)));
        assert_eq!(guard.next().map(|x| x.0), Some((6, 7)));
        assert_eq!(guard.next().map(|x| x.0), Some((6, 8)));
        assert_eq!(guard.next().map(|x| x.0), Some((5, 8)));
        assert_eq!(guard.next().map(|x| x.0), Some((4, 8)));
        assert_eq!(guard.next().map(|x| x.0), Some((3, 8)));
        assert_eq!(guard.next().map(|x| x.0), Some((2, 8)));
        assert_eq!(guard.next().map(|x| x.0), Some((1, 8)));
        assert_eq!(guard.next().map(|x| x.0), Some((1, 7)));
        assert_eq!(guard.next().map(|x| x.0), Some((2, 7)));
        assert_eq!(guard.next().map(|x| x.0), Some((3, 7)));
        assert_eq!(guard.next().map(|x| x.0), Some((4, 7)));
        assert_eq!(guard.next().map(|x| x.0), Some((5, 7)));
        assert_eq!(guard.next().map(|x| x.0), Some((6, 7)));
        assert_eq!(guard.next().map(|x| x.0), Some((7, 7)));
        assert_eq!(guard.next().map(|x| x.0), Some((7, 8)));
        assert_eq!(guard.next().map(|x| x.0), Some((7, 9)));
        assert_eq!(guard.next().map(|x| x.0), None);
    }
}
