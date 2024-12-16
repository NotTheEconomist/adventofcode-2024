pub mod errors;
pub mod parser;

pub trait Applicator<T>
where
    Self: Sized,
{
    fn apply(&self, a: T, b: T) -> T;
    fn list_all() -> Vec<Self>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operator {
    Plus,
    Mul,
}

impl Applicator<i64> for Operator {
    fn apply(&self, a: i64, b: i64) -> i64 {
        match self {
            Operator::Plus => a + b,
            Operator::Mul => a * b,
        }
    }
    fn list_all() -> Vec<Self> {
        vec![Self::Plus, Self::Mul]
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OperatorPart2 {
    Plus,
    Mul,
    Concat,
}

impl Applicator<i64> for OperatorPart2 {
    fn apply(&self, a: i64, b: i64) -> i64 {
        match self {
            OperatorPart2::Plus => a + b,
            OperatorPart2::Mul => a * b,
            OperatorPart2::Concat => {
                let mut n = 0;
                let mut sum: i64 = 0;
                for mut operand in [b, a] {
                    loop {
                        let ones = operand.rem_euclid(10);
                        let rest = operand.div_euclid(10);
                        sum += ones * 10i64.pow(n);
                        n += 1;
                        if rest == 0 {
                            break;
                        } else {
                            operand = rest
                        }
                    }
                }
                sum
            }
        }
    }

    fn list_all() -> Vec<Self> {
        vec![Self::Plus, Self::Mul, Self::Concat]
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Equation<O>
where
    O: Applicator<i64> + Clone,
{
    pub result: i64,
    pub operands: Vec<i64>,
    pub operators: Vec<O>,
}

impl<O> Equation<O>
where
    O: Applicator<i64> + Clone + std::fmt::Debug,
{
    pub fn check(&self) -> bool {
        let mut operands = self.operands.iter();
        let first = *operands.next().expect("Must always have one operand");
        operands
            .copied()
            .zip(self.operators.iter())
            .fold(first, |a, (b, op)| op.apply(a, b))
            == self.result
    }
    pub fn solve(&mut self) -> Option<&Vec<O>> {
        if !self.operators.is_empty() {
            return Some(&self.operators); // don't solve twice
        }

        let operator_count = self.operands.len() - 1;
        let mut q = O::list_all().into_iter().map(|op| vec![op]).collect::<Vec<_>>();
        while let Some(cur) = q.pop() {
            if cur.len() < operator_count {
                for op in O::list_all() {
                    let mut next = cur.clone();
                    next.push(op);
                    q.push(next);
                }
            } else {
                let mut operands = self.operands.iter();
                let first: i64 = *operands.next().expect("Must always have one operand");
                if operands
                    .copied()
                    .zip(cur.iter())
                    .fold(first, |a, (b, op)| op.apply(a, b))
                    == self.result
                {
                    self.operators = cur;
                    return Some(&self.operators);
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    // const INPUT: &str = include_str!("test_input.txt");
    use super::Applicator;

    #[test]
    fn single_equation() {
        let equation = super::Equation {
            result: 36,
            operands: vec![12, 24],
            operators: vec![super::Operator::Plus],
        };
        assert!(equation.check())
    }

    #[test]
    fn solve_equation() {
        let mut equation: super::Equation<super::Operator> = super::Equation {
            result: 36,
            operands: vec![12, 24],
            operators: vec![],
        };
        assert_eq!(equation.solve(), Some(&vec![super::Operator::Plus]));
    }

    #[test]
    fn solve_longer_equations() {
        let mut equation = super::Equation {
            result: 292,
            operands: vec![11, 6, 16, 20],
            operators: vec![],
        };
        assert_eq!(
            equation.solve(),
            Some(&vec![
                super::Operator::Plus,
                super::Operator::Mul,
                super::Operator::Plus
            ])
        )
    }

    #[test]
    fn concat_values() {
        assert_eq!(super::OperatorPart2::Concat.apply(23, 42), 2342)
    }
}
