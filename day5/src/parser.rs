use std::str::FromStr;
use std::hash::Hash;

use crate::{UpdateList, OrderingRulePair, Input};

use nom::{
    bytes::complete::tag, combinator::{eof, map_res, opt}, multi::separated_list1, sequence::separated_pair, IResult
};
use nom::character::complete::{digit1, line_ending};

fn ordering_rule<T>(s: &str) -> IResult<&str, OrderingRulePair<T>>
where
    T: FromStr,
{
    separated_pair(
        map_res(digit1, str::parse),
        tag("|"),
        map_res(digit1, str::parse),
    )(s)
}

fn update_list<T>(s: &str) -> IResult<&str, UpdateList<T>>
where
    T: FromStr,
{
    separated_list1(tag(","), map_res(digit1, str::parse))(s)
}

pub fn parse<T>(s: &str) -> Result<Input<T>, nom::Err<nom::error::Error<&str>>>
where
    T: FromStr+Eq+Hash+Copy,
{
    let (s, ordering_rules) = separated_list1(line_ending, ordering_rule)(s)?;
    let (s, _) = line_ending(s)?;
    let (s, _) = line_ending(s)?;
    let (s, update_lists) = separated_list1(line_ending, update_list)(s)?;
    let (s, _) = opt(line_ending)(s)?;
    let _ = eof(s)?;
    Ok(Input::new(ordering_rules, update_lists))
}
