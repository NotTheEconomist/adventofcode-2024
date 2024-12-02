use std::{collections::BTreeMap, convert::Infallible, fmt::Debug, ops::{Mul, Sub}, str::FromStr};

pub struct LocationIds<T>
where
    T: Ord + Sub,
{
    pub left_list: Vec<T>,
    pub right_list: Vec<T>,
    sorted_left: BTreeMap<T, usize>,
    sorted_right: BTreeMap<T, usize>,
}

// I tried to make this generic to handle multiple types of LocationId<T>, but I think this may have
// been a YAGNI mistake
impl<T> LocationIds<T>
where
    T: Sub<Output = T> + Ord + Copy + Default,
    T: Mul<Output = T>,
    T: TryFrom<usize>
{
    pub fn get_differences(&self) -> Vec<T> {
        self.sorted_left
            .iter()
            .flat_map(|(&num, &count)| std::iter::repeat(num).take(count))
            .zip(
                self.sorted_right
                    .iter()
                    .flat_map(|(&num, &count)| std::iter::repeat(num).take(count)),
            )
            .map(|(left, right)| {
                if right > left {
                    right - left
                } else {
                    left - right                }
            })
            .collect()
    }

    pub fn get_similarities(&self) -> Vec<T> {
        self.left_list.iter().map(|&num| {
            let count = *self.sorted_right.get(&num).unwrap_or(&0);
            num * count.try_into().unwrap_or_default()
        }).collect()
    }
}

impl<T> FromStr for LocationIds<T>
where
    T: Clone,
    T: Ord + Sub,
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut left_list = Vec::new();
        let mut right_list = Vec::new();
        let mut sorted_left = BTreeMap::new();
        let mut sorted_right = BTreeMap::new();
        s.trim_end().lines().for_each(|line| {
            let mut parts = line.split_whitespace().map(str::parse);

            let left: T = parts
                .next()
                .unwrap_or_else(|| panic!("All lines must have a left side, but {} does not", line))
                .expect("Left must be a valid number");
            let right: T = parts
                .next()
                .unwrap_or_else(|| panic!("All lines must have a right side, but {} does not", line))
                .expect("Right must be a valid number");

            *sorted_left.entry(left.clone()).or_insert(0usize) += 1;
            *sorted_right.entry(right.clone()).or_insert(0usize) += 1;
            left_list.push(left);
            right_list.push(right);
        });
        Ok(LocationIds {
            left_list,
            right_list,
            sorted_left,
            sorted_right,
        })
    }
}
