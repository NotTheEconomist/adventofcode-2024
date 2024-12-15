use thiserror::Error;
use std::{collections::HashMap, hash::Hash};

use crate::{OrderingRule, UpdateList};

#[derive(Error, Debug)]
pub enum Day5Error<'a, T> where T: Hash+Eq {
    #[error("failed to parse input data")]
    InputParseError(#[from] nom::Err<nom::error::Error<&'static str>>),
    #[error("cannot reorder list to match ruleset")]
    ReorderError(&'a UpdateList<T>, &'a HashMap<T, OrderingRule<T>>),
}

