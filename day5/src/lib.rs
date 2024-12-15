#![feature(slice_split_once)]
use std::collections::VecDeque;
use std::collections::hash_map::Entry;
use std::fmt::Debug;
use std::hash::Hash;
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use error::Day5Error;

pub mod error;
pub mod parser;

// Where the former comes before the latter
type OrderingRulePair<T> = (T, T);
// a list of Ts in order
type UpdateList<T> = Vec<T>;

pub fn reorder_update_list<'a, T>(
    update_list: &'a UpdateList<T>,
    ordering_rules: &'a HashMap<T, OrderingRule<T>>,
) -> Result<UpdateList<T>, error::Day5Error<'a, T>>
where
    T: Eq + Hash + Debug + Copy,
{
    let mut result: Vec<T> = update_list.to_vec();

    let mut i = 0;

    let mut changed = true;
    while changed && i < 50 {
        i += 1;
        changed = false;
        let mut new_result = result.clone();
        for (idx, value) in result.iter().enumerate() {
            if let Some(rule) = ordering_rules.get(value) {
                let (_, after) = result.split_at(idx);
                let (_, after) = after.split_at(1); // Drop the first element, which is "value"
                for (offset, other) in (1usize..).zip(after) {
                    if rule.not_after.contains(other) {
                        changed = true;
                        let other = new_result.remove(idx + offset);
                        new_result.insert(idx, other);
                    }
                }
            }
        }
        result = new_result;
    }
    if i == 50 {
        Err(error::Day5Error::ReorderError(update_list, ordering_rules))
    } else {
        Ok(result)
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct OrderingRule<T>
where
    T: Hash + Eq,
{
    not_after: HashSet<T>,
    pivot: T,
    not_before: HashSet<T>,
}

impl<T> OrderingRule<T>
where
    T: Hash + Eq,
{
    fn check(&self, update_list: &UpdateList<T>) -> bool {
        if let Some((before, after)) = update_list.split_once(|p| p == &self.pivot) {
            !(before.iter().any(|page| self.not_before.contains(page))
                || after.iter().any(|page| self.not_after.contains(page)))
        } else {
            false
        }
    }
}

#[derive(Debug)]
pub struct Input<T>
where
    T: FromStr + Hash + Eq,
{
    pub ordering_rules: HashMap<T, OrderingRule<T>>,
    pub update_lists: Vec<UpdateList<T>>,
}

impl<T> Input<T>
where
    T: FromStr + Hash + Eq + Copy,
{
    pub fn new(
        ordering_rule_pairs: Vec<OrderingRulePair<T>>,
        update_lists: Vec<UpdateList<T>>,
    ) -> Self
    where
        T: FromStr,
    {
        let mut ordering_rules = HashMap::new();
        for (before, after) in ordering_rule_pairs {
            ordering_rules
                .entry(before)
                .and_modify(|rule: &mut OrderingRule<T>| {
                    rule.not_before.insert(after);
                })
                .or_insert_with_key(|before| {
                    let not_before = HashSet::from([after]);
                    let not_after = HashSet::new();
                    let pivot = *before;
                    OrderingRule {
                        not_after,
                        pivot,
                        not_before,
                    }
                });
            ordering_rules
                .entry(after)
                .and_modify(|rule: &mut OrderingRule<T>| {
                    rule.not_after.insert(before);
                })
                .or_insert_with_key(|after| {
                    let not_before = HashSet::new();
                    let not_after = HashSet::from([before]);
                    let pivot = *after;
                    OrderingRule {
                        not_after,
                        pivot,
                        not_before,
                    }
                });
        }

        Input {
            ordering_rules,
            update_lists,
        }
    }

    pub fn valid_update_lists(&self) -> Vec<&UpdateList<T>> {
        self.update_lists
            .iter()
            .filter(|list| {
                list.iter().all(|page| {
                    if let Some(rule) = self.ordering_rules.get(page) {
                        rule.check(list)
                    } else {
                        true // Ignore pages with no rules, they are safe
                    }
                })
            })
            .collect()
    }

    pub fn invalid_update_lists(&self) -> Vec<&UpdateList<T>> {
        self.update_lists
            .iter()
            .filter(|list| {
                list.iter().any(|page| {
                    if let Some(rule) = self.ordering_rules.get(page) {
                        !rule.check(list)
                    } else {
                        false
                    }
                })
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test_input.txt");

    macro_rules! parse {
        ($s:expr) => {
            crate::parser::parse($s)
        };
    }

    #[test]
    fn naive() {
        let update_list = Vec::from([1, 2, 3, 4, 5]);
        let rules = Vec::new();
        let input = Input::new(rules, vec![update_list.clone()]);

        assert_eq!(input.valid_update_lists(), vec![&update_list]);
    }

    #[test]
    fn pass() {
        let update_list = Vec::from([1, 2, 3, 4, 5]);
        let rules = Vec::from([(2, 3)]);
        let input = Input::new(rules, vec![update_list.clone()]);

        assert_eq!(input.valid_update_lists(), vec![&update_list]);
    }

    #[test]
    fn partial_pass() {
        let pass_list = Vec::from([1, 2, 3, 4, 5]);
        let fail_list = Vec::from([1, 3, 2, 4, 5]);
        let rules = Vec::from([(2, 3)]);
        let input = Input::new(rules, vec![pass_list.clone(), fail_list.clone()]);

        assert_eq!(input.valid_update_lists(), vec![&pass_list]);
    }

    #[test]
    fn complex_test() {
        let input: Input<u8> = parse!(
            "\
2|3
3|4
4|6
12|2
12|3

1,2,3,4,5
2,3,4,5,6
3,4,5,9,12
12,2,4,3,5
12,2,3,4,5"
        )
        .unwrap();

        assert_eq!(input.valid_update_lists(), vec![
            &vec![1, 2, 3, 4, 5],
            &vec![2, 3, 4, 5, 6],
            &vec![12, 2, 3, 4, 5]
        ]);
    }
    #[test]
    fn reorder_single() {
        let list = vec![3, 4, 5, 9, 12];
        let rules = vec![(3, 4), (4, 5), (5, 9), (12, 2), (12, 3)];
        let input = Input::new(rules, vec![list.clone()]);

        let rules = input.ordering_rules;
        let reordered = reorder_update_list(&list, &rules);
        assert_eq!(reordered.unwrap(), vec![12, 3, 4, 5, 9])
    }
}
