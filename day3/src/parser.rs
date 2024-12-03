use nom::{
    IResult,
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, char, digit1},
    combinator::{all_consuming, map, map_res, rest, value},
    multi::{many_till, many1},
    sequence::{delimited, preceded, separated_pair},
};

#[derive(Debug)]
enum ParsedAtom {
    Conditional(crate::Conditional),
    Mul(crate::Mul),
}

fn mul(input: &str) -> IResult<&str, crate::Mul> {
    map(
        preceded(
            tag("mul"),
            delimited(
                char('('),
                separated_pair(
                    map_res(digit1, str::parse),
                    char(','),
                    map_res(digit1, str::parse),
                ),
                char(')'),
            ),
        ),
        |(x, y)| crate::Mul { x, y },
    )(input)
}

fn conditional(input: &str) -> IResult<&str, crate::Conditional> {
    alt((
        value(crate::Conditional::Dont, tag("don't()")),
        value(crate::Conditional::Do, tag("do()")),
    ))(input)
}

fn atoms(input: &str) -> IResult<&str, Vec<ParsedAtom>> {
    let mul_parser = map(mul, ParsedAtom::Mul);
    let conditional_parser = map(conditional, ParsedAtom::Conditional);
    let (input, many_results) =
        many1(many_till(anychar, alt((conditional_parser, mul_parser))))(input)?;
    let atoms = many_results.into_iter().map(|(_, atom)| atom).collect();
    Ok((input, atoms))
}

pub fn parse(
    input: &str,
    use_conditionals: bool,
) -> Result<Vec<crate::Mul>, crate::error::ParseError> {
    let (input, atoms) = atoms(input)?;
    let _ = all_consuming(rest)(input)?;
    let mut muls: Vec<crate::Mul> = Vec::new();
    let mut r#do = true;
    for atom in atoms {
        if use_conditionals {
            match (r#do, atom) {
                (true, ParsedAtom::Mul(mul)) => muls.push(mul),
                (_, ParsedAtom::Conditional(crate::Conditional::Dont)) => r#do = false,
                (_, ParsedAtom::Conditional(crate::Conditional::Do)) => r#do = true,
                (false, ParsedAtom::Mul(_)) => {}
            }
        } else if let ParsedAtom::Mul(mul) = atom {
            muls.push(mul)
        }
    }
    Ok(muls)
}

#[cfg(test)]
mod error {
    use std::assert_matches::assert_matches;

    use super::*;
    use crate::Mul;
    #[test]
    fn parse_mul() {
        let input = "mul(1,2)";
        assert_eq!(mul(input), Ok(("", Mul { x: 1, y: 2 })))
    }
    #[test]
    fn parse_muls_with_extras() {
        let input = "abcmul(123,456)xyzmul(21,1)123";
        assert_eq!(
            parse(input, false),
            Ok(vec![Mul { x: 123, y: 456 }, Mul { x: 21, y: 1 }])
        )
    }
    #[test]
    fn parse_conditional() {
        let input = "don't()do()";
        assert_matches!(conditional(input), Ok(("do()", crate::Conditional::Dont)))
    }
    #[test]
    fn parse_atoms() {
        let input = "adon't()bdo()cmul(21,44)";
        let (_, result) = atoms(input).unwrap();
        assert_matches!(result.as_slice(), [
            ParsedAtom::Conditional(crate::Conditional::Dont),
            ParsedAtom::Conditional(crate::Conditional::Do),
            ParsedAtom::Mul(Mul { x: 21, y: 44 })
        ])
    }
}
