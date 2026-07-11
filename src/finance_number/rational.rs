use std::{num::NonZeroU128, ops::Add};

use super::integer::FinanceInt;
/*
#[derive(Debug)]
pub struct FinanceRational {
    n: FinanceInt,
    d: NonZeroU128,
}

impl FinanceRational {
    fn new(n: i128, d: NonZeroU128) -> Result<Self, ArithmeticError> {
        todo!("not impl yet");
    }
}

#[derive(Debug)]
pub enum ArithmeticError {
    Div0,
    Overflow,
    Underflow,
}

impl Add<FinanceRational> for FinanceRational {
    type Output = Result<Self, ArithmeticError>;

    fn add(self, rhs: Self) -> Self::Output {
        todo!("impl add operation for rational number");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let a = FinanceRational::new(1, 2).expect("should not overflow");
        assert_eq!(a.n, FinanceInt(1));
        assert_eq!(a.d, FinanceInt(2));
    }

    #[test]
    fn test_new_div0() {
        let a = FinanceRational::new(1, 0); //should div by zero
        assert!(matches!(a, Err(ArithmeticError::Div0)));
    }

    #[test]
    fn test_new_negative_denominator() {
        let a = FinanceRational::new(1, -2).expect("constant value");
        assert_eq!(a.n, FinanceInt(-1));
        assert_eq!(a.d, FinanceInt(2));
    }

    #[test]
    fn test_add_same_denominator() {
        let a = FinanceRational::new(1, 2).expect("constant value");
        let b = FinanceRational::new(1, 2).expect("constant value");
        let c = (a + b).expect("should not overflow");
        assert_eq!(c.n, FinanceInt(1));
        assert_eq!(c.d, FinanceInt(1));
    }

    #[test]
    fn test_add_diff_denominator() {
        let a = FinanceRational::new(1, 2).expect("constant value");
        let b = FinanceRational::new(1, 4).expect("constant value");
        let c = (a + b).expect("should not overflow");
        assert_eq!(c.n, FinanceInt(3));
        assert_eq!(c.d, FinanceInt(4));
    }

    fn test_add_diff_denominator_anti_overflow() {
        let a = FinanceRational::new(1, i128::MAX).expect("constant value");
        let b = FinanceRational::new(1, i128::MAX - 1).expect("constant value");
        let c = (a + b).expect("should not overflow");
        assert_eq!(c.n, FinanceInt(1));
        assert_eq!(c.d, FinanceInt(1));
    }
}
    */
