#[derive(Debug, Copy, Clone)]
enum Direction {
    Ascending,
    Descending,
}

impl Direction {
    /// opposite was used in the "with_dampener" state machine solution
    // fn opposite(&self) -> Self {
    //     match self {
    //         Direction::Ascending => Direction::Descending,
    //         Direction::Descending => Direction::Ascending,
    //     }
    // }
    fn safely(&self, prev: u8, next: u8) -> bool {
        match self {
            Direction::Ascending => next > prev && next - prev <= 3,
            Direction::Descending => next < prev && prev - next <= 3,
        }
    }

    fn from(prev: u8, next: u8) -> Option<Self> {
        match next.cmp(&prev) {
            std::cmp::Ordering::Less if prev - next <= 3 => Some(Direction::Descending),
            std::cmp::Ordering::Greater if next - prev <= 3 => Some(Direction::Ascending),
            _ => None,
        }
    }
}

pub fn is_safe(level: &[u8]) -> bool {
    level
        .iter()
        .fold(
            (true, None::<Direction>, None),
            |(is_safe, direction, prev), &next| match (is_safe, direction, prev) {
                (false, _, _) => (false, None, None),
                (true, Some(dir), Some(prev)) => (dir.safely(prev, next), direction, Some(next)),
                (true, None, None) => (true, None, Some(next)),
                (true, None, Some(prev)) => {
                    if let Some(dir) = Direction::from(prev, next) {
                        (true, Some(dir), Some(next))
                    } else {
                        (false, None, None)
                    }
                }
                (true, Some(_), None) => unreachable!(),
            },
        )
        .0
}

pub fn is_safe_with_problem_dampener(level: &[u8]) -> bool {
    if is_safe(level) { true } else {

        (0..level.len()).map(|i| {
            let before = &level[..i];
            let after = &level[i + 1usize..];
            [before, after].concat()
        }).any(|modified_level| is_safe(&modified_level))
    }
}

// There's a solution here using a similar state machine stepper logic, but tracking more
// individual states. This should allow a solution in O(n) complexity. However, I couldn't get the
// edge cases working right in a reasonable amount of time. The conflicting cases were:
//
// 57 55 58 61 63 64 65 67
//
// Where dropping the first element would allow the series to be ascending safely. I could not find
// a safe way to checking for that in O(n) time without breaking one of the test cases:
//
// 1 3 2 4 5
//
// Where dropping the _second_ element would allow the series to continue to ascend safely. Since
// dropping the first element will also allow the series to continue _descending_ safely briefly, I
// end up having to fork the logic and lose O(n) immediately.
//
// I'm confident that there's a solution down this path, but I'm not experienced enough to solve
// it. Instead I threw more compute at it and just calculated each level were it missing any one of its
// elements, lazily checking if any of those variations were safe instead.
//
// I leave my aborted state machine (in all its unreadable complexity -- I hadn't gotten around to
// cleaning any of this up before abandoning course) in case anyone would like to follow along with
// my logic
//
//
// pub fn is_safe_with_problem_dampener(level: &[u8]) -> bool {
//     let result = level.iter().fold(
//         (true, false, true, None::<Direction>, None, None),
//         |(is_safe, dampener_used, can_change_course, direction, prevprev, prev), &next| match (
//             is_safe,
//             dampener_used,
//             can_change_course,
//             direction,
//             prevprev,
//             prev,
//         ) {
//             (_, _, _, _, Some(_), None) => unreachable!("Can't have a prevprev without a prev"),
//             (false, false, _, None, None, _) => {
//                 unreachable!("Can't fail (the first time) before getting to two values")
//             }
//             (true, _, _, None, Some(_), Some(_)) => {
//                 unreachable!("Can't have a safe level with two values without a direction")
//             }
//             (_, _, _, Some(_), None, Some(_)) | (_, _, _, Some(_), None, None) => {
//                 unreachable!("Can't have a direction without two values")
//             }
//             (_, _, _, None, Some(_), Some(_)) => unreachable!("Can't have two values without a direction"),
//             (false, true, _, _, _, _) => (false, true, false, None, None, None),
//             // dampener can be effectively used
//             (false, false, false, Some(dir), Some(prevprev), _) => (
//                 dir.safely(prevprev, next),
//                 true,
//                 can_change_course,
//                 Some(dir),
//                 Some(prevprev),
//                 Some(next),
//             ),
//             // If we can still change course, give it a try
//             (false, false, true, _, Some(prevprev), Some(_)) => {
//                 if let Some(dir) = Direction::from(prevprev, next) {
//                     (
//                         dir.safely(prevprev, next),
//                         true,
//                         true,
//                         Some(dir),
//                         Some(prevprev),
//                         Some(next),
//                     )
//                 } else {
//                     (false, true, can_change_course, None, None, None)
//                 }
//             }
//             (true, _, _, None, None, None) => (
//                 true,
//                 dampener_used,
//                 can_change_course,
//                 None,
//                 None,
//                 Some(next),
//             ),
//             (true, _, _, None, None, Some(prev)) => {
//                 if let Some(dir) = Direction::from(prev, next) {
//                     (
//                         true,
//                         dampener_used,
//                         can_change_course,
//                         Some(dir),
//                         Some(prev),
//                         Some(next),
//                     )
//                 } else {
//                     (
//                         false,
//                         dampener_used,
//                         can_change_course,
//                         None,
//                         Some(prev),
//                         Some(next),
//                     )
//                 }
//             }
//             (true, _, true, Some(dir), Some(_), Some(prev)) => {
//                 if dir.safely(prev, next) {
//                     (
//                         true,
//                         dampener_used,
//                         false,
//                         Some(dir),
//                         Some(prev),
//                         Some(next),
//                     )
//                 } else if dir.opposite().safely(prev, next) {
//                     (
//                         true,
//                         true,
//                         false,
//                         Some(dir.opposite()),
//                         Some(prev),
//                         Some(next),
//                     )
//                 } else {
//                     (
//                         false,
//                         dampener_used,
//                         true,
//                         Some(dir),
//                         Some(prev),
//                         Some(next),
//                     )
//                 }
//             }
//             (true, _, false, Some(dir), Some(_), Some(prev)) => {
//                 if dir.safely(prev, next) {
//                     (
//                         true,
//                         dampener_used,
//                         false,
//                         Some(dir),
//                         Some(prev),
//                         Some(next),
//                     )
//                 } else {
//                     (
//                         false,
//                         dampener_used,
//                         false,
//                         Some(dir),
//                         Some(prev),
//                         Some(next),
//                     )
//                 }
//             }
//         },
//     );
//     result.0 || !result.1 // catches the case where the final value should be ignored
// }
