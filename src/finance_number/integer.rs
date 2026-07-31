use crate::finance_number::positive::FinancePositive;

use super::ArithmeticError;
use std::ops::{Add, Div, Mul, Neg, Sub};

/// A lightweight wrapper around i128 that provides checked arithmetic operations for finance.
///
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
pub struct FinanceInt(pub i128);

impl From<i128> for FinanceInt {
    fn from(value: i128) -> Self {
        Self(value)
    }
}

impl TryFrom<FinancePositive> for FinanceInt {
    type Error = ArithmeticError;

    fn try_from(value: FinancePositive) -> Result<Self, Self::Error> {
        if value.0.get() > i128::MAX as u128 {
            Err(ArithmeticError::Overflow)
        } else {
            Ok(Self(value.0.get() as i128))
        }
    }
}

impl TryFrom<u128> for FinanceInt {
    type Error = ArithmeticError;
    fn try_from(value: u128) -> Result<Self, Self::Error> {
        if value > i128::MAX as u128 {
            Err(ArithmeticError::VaueOutOfBound)
        } else {
            Ok(Self(value as i128))
        }
    }
}

impl Add<Self> for FinanceInt {
    type Output = Result<Self, ArithmeticError>;
    /// Perform checked add operation.
    /// # Returns
    ///
    /// Returens Add result if not overflow.
    ///
    /// # Errors
    ///
    /// Returns `ArithmeticError::Overflow` if the result overflows.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use double_ledger::finance_number::FinanceInt;
    /// let a = FinanceInt(1);
    /// let b = FinanceInt(2);
    /// let c = a + b;
    /// assert_eq!(c.unwrap(), FinanceInt(3));
    /// ```
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
        if rhs.0 == 0 {
            return Err(ArithmeticError::Div0);
        }
        let rem = self.0.checked_rem(rhs.0).ok_or(ArithmeticError::Overflow)?;
        //division checked by checked_rem, unchecked division after rem is safe.
        if rem != 0 {
            return Err(ArithmeticError::Indivisible(self.0 / rhs.0, rem));
        }
        Ok(Self(self.0 / rhs.0))
    }
}

impl Neg for FinanceInt {
    type Output = Result<Self, ArithmeticError>;
    fn neg(self) -> Self::Output {
        self.0
            .checked_neg()
            .map(Self)
            .ok_or(ArithmeticError::Overflow)
    }
}

impl FinanceInt {
    pub fn gcd(a: Self, b: Self) -> Self {
        Self(num::integer::gcd(a.0, b.0))
    }

    pub fn lcm(a: Self, b: Self) -> Self {
        Self(num::integer::lcm(a.0, b.0))
    }
}
#[cfg(test)]
pub(crate) mod tests {
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
    fn test_div_indivisivle() {
        let a = FinanceInt(11);
        let b = FinanceInt(2);
        let c = a / b; //should return error;
        assert!(matches!(c, Err(ArithmeticError::Indivisible(5, 1))));
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
