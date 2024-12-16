use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending},
    combinator::{all_consuming, map_res},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

use crate::errors;

fn result(input: &str) -> IResult<&str, i64> {
    map_res(digit1, str::parse)(input)
}

fn operands(input: &str) -> IResult<&str, Vec<i64>> {
    separated_list1(tag(" "), map_res(digit1, str::parse))(input)
}

fn equation_without_operators<O: super::Applicator<i64>+Clone>(input: &str) -> IResult<&str, crate::Equation<O>> {
    let (input, (result, operands)) = separated_pair(result, tag(": "), operands)(input)?;
    Ok((
        input,
        crate::Equation {
            result,
            operands,
            operators: Vec::new(),
        },
    ))
}

pub fn parse<O: super::Applicator<i64>+Clone>(input: &'static str) -> Result<Vec<crate::Equation<O>>, errors::Day7Error> {
    all_consuming(separated_list1(line_ending, equation_without_operators))(input.trim_end())
        .map_err(errors::Day7Error::InputParseError)
        .map(|x| x.1)
}
