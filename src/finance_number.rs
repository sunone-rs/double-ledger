use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Neg, Sub},
};

use crate::finance_number::ArithmeticError::Div0;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
pub struct FinanceInt(i128);

impl Add<Self> for FinanceInt {
    type Output = Result<Self, ArithmeticError>;

    fn add(self, rhs: Self) -> Self::Output {
        self.0
            .checked_add(rhs.0)
            .map(Self)
            .ok_or(ArithmeticError::Overflow)
    }
}

impl Sub<Self> for FinanceInt {
    type Output = Result<Self, ArithmeticError>;

    fn sub(self, rhs: Self) -> Self::Output {
        self.0
            .checked_sub(rhs.0)
            .map(Self)
            .ok_or(ArithmeticError::Overflow)
    }
}

impl Mul<Self> for FinanceInt {
    type Output = Result<Self, ArithmeticError>;

    fn mul(self, rhs: Self) -> Self::Output {
        self.0
            .checked_mul(rhs.0)
            .map(Self)
            .ok_or(ArithmeticError::Overflow)
    }
}

impl Div<Self> for FinanceInt {
    type Output = Result<Self, ArithmeticError>;

    fn div(self, rhs: Self) -> Self::Output {
        if rhs.0==0{
            return Err(Div0);
        }
        self.0.checked_div(rhs.0).map(Self).ok_or(ArithmeticError::Overflow)
    }
}

impl Neg for FinanceInt {
    type Output = Result<Self, ArithmeticError>;
    fn neg(self) -> Self::Output {
        self.0.checked_neg().map(Self).ok_or(ArithmeticError::Overflow)
    }
}


#[derive(Debug)]
pub enum ArithmeticError {
    Overflow,
    Div0,
}

impl std::error::Error for ArithmeticError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.source()
    }
}

impl Display for ArithmeticError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let a = FinanceInt(1);
        let b = FinanceInt(2);
        let c = (a + b).expect("should not overflow");
        assert_eq!(c, FinanceInt(3));
    }

    #[test]
    fn test_add_overflow() {
        let a = FinanceInt(1);
        let b = FinanceInt(i128::MAX);
        let c = a + b;
        assert!(matches!(c, Err(ArithmeticError::Overflow)));
    }

    #[test]
    fn test_sub() {
        let a = FinanceInt(3);
        let b = FinanceInt(1);
        let c = (a - b).expect("should not overflow");
        assert_eq!(c, FinanceInt(2));
    }

    #[test]
    fn test_sub_overflow() {
        let a = FinanceInt(i128::MIN);
        let b = FinanceInt(1);
        let c = a - b; //should overflow
        assert!(matches!(c, Err(ArithmeticError::Overflow)));
    }

    #[test]
    fn test_mul() {
        let a = FinanceInt(2);
        let b = FinanceInt(3);
        let c = (a * b).expect("should not overflow");
        assert_eq!(c, FinanceInt(6));
    }

    #[test]
    fn test_mul_overflow() {
        let a = FinanceInt(i128::MAX);
        let b = FinanceInt(2);
        let c = a * b; //should overflow
        assert!(matches!(c, Err(ArithmeticError::Overflow)));
    }

    #[test]
    fn test_div() {
        let a = FinanceInt(6);
        let b = FinanceInt(2);
        let c = (a / b).expect("should not overflow");
        assert_eq!(c, FinanceInt(3));
    }

    #[test]
    fn test_div_zero() {
        let a = FinanceInt(1);
        let b = FinanceInt(0);
        let c = a / b; //should div by zero
        assert!(matches!(c, Err(ArithmeticError::Div0)));
    }

    #[test]
    fn test_div_overflow() {
        let a = FinanceInt(i128::MIN);
        let b = FinanceInt(-1);
        let c = a / b; //should overflow
        assert!(matches!(c, Err(ArithmeticError::Overflow)));
    }

    #[test]
    fn test_neg() {
        let a = FinanceInt(1);
        let c = a.neg().expect("should not overflow");
        assert_eq!(c, FinanceInt(-1));
    }

    #[test]
    fn test_neg_overflow() {
        let a = FinanceInt(i128::MIN);
        let c = a.neg();
        assert!(matches!(c, Err(ArithmeticError::Overflow)));
    }
}
