use super::ArithmeticError;
use std::num::NonZeroU128;
use std::ops::{Add, Div, Mul, Sub};

/// A non-zero positive integer for finance calculation.
/// # Invariants
/// * Always non-zero value.
/// * Always positive value.
/// # Safety
/// * Always checked calculation performed.
/// * Not be panic
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
pub struct FinancePositive(pub NonZeroU128);

impl From<NonZeroU128> for FinancePositive {
    /// Convert NonZeroU128 to FinancePositive.
    /// # Details
    /// This function converts a NonZeroU128 to a FinancePositive.
    /// # Safety
    /// This method never fails, because .
    fn from(value: NonZeroU128) -> Self {
        Self(value)
    }
}

impl TryFrom<u128> for FinancePositive {
    type Error = ArithmeticError;
    /// Convert u128 to FinancePositive.
    /// # Details
    /// This function converts a u128 to a FinancePositive.
    /// # Errors
    /// This function returns ZeroOnNonZeroValue when input is 0
    fn try_from(value: u128) -> Result<Self, Self::Error> {
        NonZeroU128::new(value)
            .map(Self)
            .ok_or(ArithmeticError::VaueOutOfBound)
    }
}

impl TryFrom<i128> for FinancePositive {
    type Error = ArithmeticError;
    /// Convert i128 to FinancePositive.
    /// # Details
    /// This function converts a i128 to a FinancePositive.
    /// # Errors
    /// This function returns ZeroOnNonZeroValue when input is 0
    /// This function returns Overflow when input is negative
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        if value <= 0 {
            return Err(ArithmeticError::VaueOutOfBound);
        }
        Self::try_from(value as u128)
    }
}

impl Add<Self> for FinancePositive {
    type Output = Result<Self, ArithmeticError>;
    /// Perform add operation for two finance positive numbers.
    /// # Details
    /// This function adds two finance positive numbers and returns the result.
    /// # Errors
    /// This function returns Overflow if the result overflows.
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        self.0
            .checked_add(rhs.0.into())
            .map(Self)
            .ok_or(ArithmeticError::Overflow)
    }
}

impl Sub<Self> for FinancePositive {
    type Output = Result<Self, ArithmeticError>;
    /// Perform subtraction operation for two finance positive numbers.
    /// # Details
    /// This function subtracts two finance positive numbers and returns the result.
    /// # Errors
    /// This function returns Overflow when right hand value is equal or greater than left hand value.
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        let result = self.0.get().checked_sub(rhs.0.get());
        match result {
            Some(value) if value > 0 => Self::try_from(value),
            _ => Err(ArithmeticError::Overflow),
        }
    }
}

impl Mul<Self> for FinancePositive {
    type Output = Result<Self, ArithmeticError>;
    /// Perform multiplication operation for two finance positive numbers.
    /// # Details
    /// This function multiplies two finance positive numbers and returns the result.
    /// # Errors
    /// This function returns Overflow if the result overflows.
    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        self.0
            .checked_mul(rhs.0)
            .map(Self)
            .ok_or(ArithmeticError::Overflow)
    }
}

impl Div<Self> for FinancePositive {
    type Output = Result<Self, ArithmeticError>;
    /// Perform division operation for two finance positive numbers.
    /// # Details
    /// This function divides two finance positive numbers and returns the result.
    /// # Errors
    /// This function returns Indivisible if the result is not an integer.
    /// This function returns ZeroOnNonZeroValue if the result is 0.
    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        let result = (self.0.get() / rhs.0.get(), self.0.get() % rhs.0.get());

        if result.1 != 0 {
            return Err(ArithmeticError::Indivisible(
                result.0 as i128,
                result.1 as i128,
            ));
        }
        if result.0 == 0 {
            return Err(ArithmeticError::VaueOutOfBound);
        }
        Ok(FinancePositive(NonZeroU128::new(result.0).unwrap()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::num::NonZeroU128;

    #[test]
    fn test_add() {
        let a = FinancePositive(NonZeroU128::new(1).expect("literal value, should not fail"));
        let b = FinancePositive(NonZeroU128::new(2).expect("literal value, should not fail"));
        let c = (a + b).expect("should not overflow");
        assert_eq!(
            c.0,
            NonZeroU128::new(3).expect("literal value, should not fail")
        );
    }

    #[test]
    fn test_add_same() {
        let a = FinancePositive(NonZeroU128::new(2).expect("literal value, should not fail"));
        let b = FinancePositive(NonZeroU128::new(2).expect("literal value, should not fail"));
        let c = (a + b).expect("should not overflow");
        assert_eq!(
            c.0,
            NonZeroU128::new(4).expect("literal value, should not fail")
        );
    }

    #[test]
    fn test_add_big() {
        let a = FinancePositive(
            NonZeroU128::new(5000000000000000000).expect("literal value, should not fail"),
        );
        let b = FinancePositive(
            NonZeroU128::new(5000000000000000000).expect("literal value, should not fail"),
        );
        let c = (a + b).expect("should not overflow");
        assert_eq!(
            c.0,
            NonZeroU128::new(10000000000000000000).expect("literal value, should not fail")
        );
    }

    #[test]
    fn test_add_overflow() {
        let a = FinancePositive(NonZeroU128::MAX);
        let b = FinancePositive(NonZeroU128::new(1).expect("literal value, should not fail"));
        let c = a + b;
        assert!(matches!(c, Err(ArithmeticError::Overflow)));
    }

    #[test]
    fn test_mul() {
        let a = FinancePositive(NonZeroU128::new(2).expect("literal value, should not fail"));
        let b = FinancePositive(NonZeroU128::new(3).expect("literal value, should not fail"));
        let c = a * b;
        assert_eq!(
            c.expect("should not overflow").0,
            NonZeroU128::new(6).unwrap()
        );
    }

    #[test]
    fn test_mul_same() {
        let a = FinancePositive(NonZeroU128::new(6).expect("literal value, should not fail"));
        let b = FinancePositive(NonZeroU128::new(6).expect("literal value, should not fail"));

        let c = a * b;
        assert_eq!(
            c.expect("should not overflow").0,
            NonZeroU128::new(36).unwrap()
        );
    }

    #[test]
    fn test_mul_big() {
        let a = FinancePositive(NonZeroU128::MAX);
        let b = FinancePositive(NonZeroU128::new(1).expect("literal value, should not fail"));
        let c = a * b;
        assert_eq!(c.expect("should not overflow").0, NonZeroU128::MAX);
    }

    #[test]
    fn test_mul_overflow() {
        let a = FinancePositive(NonZeroU128::MAX);
        let b = FinancePositive(NonZeroU128::new(2).expect("literal value, should not fail"));
        let c = a * b;
        assert!(matches!(c, Err(ArithmeticError::Overflow)));
    }

    #[test]
    fn test_div() {
        let a = FinancePositive(NonZeroU128::new(4).expect("literal value, should not fail"));
        let b = FinancePositive(NonZeroU128::new(2).expect("literal value, should not fail"));
        let c = a / b;
        assert_eq!(
            c.expect("should not overflow"),
            FinancePositive(NonZeroU128::new(2).expect("literal value, should not fail"))
        );
    }

    #[test]
    fn test_div_by_one() {
        let a = FinancePositive(NonZeroU128::new(4).expect("literal value, should not fail"));
        let b = FinancePositive(NonZeroU128::new(1).expect("literal value, should not fail"));
        let c = a / b;
        assert_eq!(
            c.expect("should not overflow"),
            FinancePositive(NonZeroU128::new(4).expect("literal value, should not fail"))
        );
    }

    #[test]
    fn test_div_indivisible() {
        let a = FinancePositive(NonZeroU128::new(4).expect("literal value, should not fail"));
        let b = FinancePositive(NonZeroU128::new(3).expect("literal value, should not fail"));
        let c = a / b;
        assert!(matches!(c, Err(ArithmeticError::Indivisible(1, 1))));
    }
}
