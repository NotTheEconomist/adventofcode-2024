#![feature(assert_matches)]
pub mod parser;
mod error;

#[derive(Debug, PartialEq, Eq)]
pub struct Mul {
    pub x: u16,
    pub y: u16
}

impl From<Mul> for u64 {
    fn from(value: Mul) -> Self {
        value.x as u64 * value.y as u64
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Conditional {
    Do,
    Dont
}
