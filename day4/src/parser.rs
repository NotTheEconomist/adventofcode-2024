pub fn parse(s: &str) -> Vec<((usize, usize), char)> {
    s.lines().enumerate().flat_map(|(y, line)| {
        line.char_indices().filter_map(move |(x, c)| if matches!(c, 'X'|'M'|'A'|'S'|'x'|'m'|'a'|'s') { Some(((x, y), c)) } else { None })
    }).collect()
}

#[cfg(test)]
mod parser_tests {
    use super::*;
    use std::assert_matches::assert_matches;
    #[test]
    fn test_noparse() {
        let result = parse("bcdefghijklnopqrtuvwyz");  // no output
        assert_eq!(result, Vec::new())
    }
    #[test]
    fn test_columns() {
        let should_be_0 = parse("a");
        let should_be_2 = parse("__a");
        let should_be_22 = parse("______________________a");
        assert_matches!(should_be_0.as_slice(), &[((0, _), 'a')]);
        assert_matches!(should_be_2.as_slice(), &[((2, _), 'a')]);
        assert_matches!(should_be_22.as_slice(), &[((22, _), 'a')]);
    }

    #[test]
    fn test_rows() {
        let should_be_0 = parse("192308a182394");
        let should_be_2 = parse("\n\n192308a182394");
        let should_be_22 = parse("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n192308a182394");
        assert_matches!(should_be_0.as_slice(), &[((_, 0), 'a')]);
        assert_matches!(should_be_2.as_slice(), &[((_, 2), 'a')]);
        assert_matches!(should_be_22.as_slice(), &[((_, 22), 'a')]);
    }
}
