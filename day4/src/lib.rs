#![feature(assert_matches)]

use std::collections::HashMap;

pub mod parser;

pub fn filter_xmas(loc: (usize, usize), map: &HashMap<(usize, usize), char>) -> Vec<(usize, usize)> {
    fn helper(letters: Option<[(usize, usize); 4]>, map: &HashMap<(usize, usize), char>) -> Option<[(usize, usize); 4]> {
        if let Some(letters) = letters {
            let values: String = letters
                .into_iter()
                .filter_map(|loc| map.get(&loc))
                .collect();
            match values.as_str() {
                "xmas" | "XMAS" | "SAMX" | "samx" => Some(letters),
                _ => None
            }
        } else { None }
    }
    fn e((x, y): (usize, usize)) -> Option<[(usize, usize); 4]> {
        Some([(x, y), (x + 1, y), (x + 2, y), (x + 3, y)])
    }
    fn se((x, y): (usize, usize)) -> Option<[(usize, usize); 4]> {
        Some([(x, y), (x + 1, y + 1), (x + 2, y + 2), (x + 3, y + 3)])
    }
    fn s((x, y): (usize, usize)) -> Option<[(usize, usize); 4]> {
        Some([(x, y), (x, y + 1), (x, y + 2), (x, y + 3)])
    }
    fn sw((x, y): (usize, usize)) -> Option<[(usize, usize); 4]> {
        Some([
            (x, y),
            (x.checked_sub(1)?, y + 1),
            (x.checked_sub(2)?, y + 2),
            (x.checked_sub(3)?, y + 3),
        ])
    }

    match map.get(&loc) {
        Some(&'s') | Some(&'x') | Some(&'S') | Some(&'X') => {
            let east = e(loc);
            let southeast = se(loc);
            let south = s(loc);
            let southwest = sw(loc);
            [east, southeast, south, southwest].into_iter().filter_map(|locs| helper(locs, map)).flatten().collect()
        }
        _ => Vec::new(),
    }
}

pub fn filter_x_mas(loc: (usize, usize), map: &HashMap<(usize, usize), char>) -> Vec<(usize, usize)> {
    fn get_neighbors((x, y): (usize, usize)) -> Option<[(usize, usize); 4]> {
        // [nw, ne, sw, se]
        Some([(x.checked_sub(1)?, y.checked_sub(1)?),
            (x.checked_add(1)?, y.checked_sub(1)?),
            (x.checked_sub(1)?, y.checked_add(1)?),
            (x.checked_add(1)?, y.checked_add(1)?)])
    }
    // early out if we're not looking at the center letter
    if !matches!(map.get(&loc), Some(&'A')) { return Vec::new() }
    if let Some(neighbors) = get_neighbors(loc) {
        match neighbors.iter().filter_map(|loc| map.get(loc)).collect::<String>().as_str() {
            "MMSS" | "MSMS" | "SSMM" | "SMSM" => std::iter::once(loc).chain(neighbors).collect(),
            _ => Vec::new()
        }
    } else {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! parse {
        ($grid:expr) => {{
            crate::parser::parse($grid).into_iter().collect::<HashMap<_, _>>()
        }};
    }

    macro_rules! assert_xmas {
        ($loc:expr, $grid:expr) => {
            assert!(filter_xmas($loc, &$grid).len() > 0)
        };
        ($loc:expr, $grid:expr, $count:literal) => {
            let result = filter_xmas($loc, &$grid);
            let expected_length = $count * 4;
            assert!(result.len() == expected_length, "filter_xmas expected to find {} valid XMASes, got {:?} (result len {:?}, expected len {})", $count, (result.len() - 1) / 3, result.len(), expected_length)
        };
    }

    macro_rules! assert_x_mas {
        ($loc:expr, $grid:expr) => {
            assert!(filter_x_mas($loc, &$grid).len() > 0)
        };
    }

    #[test]
    fn east() {
        assert_xmas!((0, 0), parse!("XMAS"));
    }
    #[test]
    fn west() {
        assert_xmas!((0, 0), parse!("SAMX"));
    }
    #[test]
    fn south() {
        assert_xmas!((0, 0), parse!("X\nM\nA\nS"));
    }
    #[test]
    fn north() {
        assert_xmas!((0, 0), parse!("S\nA\nM\nX"));
    }
    #[test]
    fn grid() {
        let grid = parse!("XMAS\nM__A\nA__M\nSAMX");
        assert_xmas!((0, 0), grid);
        assert_xmas!((0, 3), grid);
        assert_xmas!((3, 0), grid);
    }
    #[test]
    fn inverted_v() {
        let grid = parse!("\
___X___
__M_M__
_A___A_
S_____S");
        assert_xmas!((3, 0), grid, 2);
    }
    #[test]
    fn x_mas() {
        let grid = parse!("\
S_SM_S
_A__A_
M_MM_S
M_MS_M
_A__A_
S_SS_M");
        assert_x_mas!((1, 1), grid);
        assert_x_mas!((4, 1), grid);
        assert_x_mas!((1, 4), grid);
        assert_x_mas!((4, 4), grid);
    }
}
