use super::integer::FinanceInt;
use super::positive::FinancePositive;
use crate::finance_number::ArithmeticError;
use num::traits::Inv;
use num_integer::gcd;
use std::{
    num::NonZeroU128,
    ops::{Add, Div, Mul, Neg, Sub},
};

///Rational number for finance calculation.
/// # Invariants
/// * The numerator and denominator are coprime.
/// * denominator is always has always non-zero positive value.
/// * Always simplest form of the rational number.
/// # Safety
/// * Always checked calculation performed.
/// * Not be panic
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct FinanceRational {
    n: FinanceInt,
    d: FinancePositive,
}

impl FinanceRational {
    /// Create a rational number from an integer.
    /// # Summary
    /// This function creates a finance rational number from a finance integer and a finance positive number.
    /// This value can be used in subsequent financial calculations.
    /// # Details
    /// The input is numerator and denominator pair. The rational number is reduced to its simplest form.
    /// # Safety
    /// This method never fails, Because all of the inputs are guaranteed to be valid.
    pub fn new(n: FinanceInt, d: FinancePositive) -> Self {
        let n = n.0;
        let d = d.0.get() as i128;
        let common_factor = gcd(n, d);
        if n == 0 {
            return Self {
                n: FinanceInt(0),
                d: FinancePositive(NonZeroU128::new(1).unwrap()),
            };
        }
        Self {
            n: FinanceInt(n / common_factor),
            d: FinancePositive(NonZeroU128::new((d / common_factor).try_into().unwrap()).unwrap()),
        }
    }
}

impl Add<FinanceRational> for FinanceRational {
    type Output = Result<Self, ArithmeticError>;
    /// perform add operation for two finance rational numbers.
    /// # Summary
    /// This function adds two finance rational numbers and returns the result.
    /// # Details
    /// The input rational numbers are added together, and the result is reduced to its simplest form.
    /// # Errors
    /// This method may return `ArithmeticError::Overflow` if the result overflows `i128` or
    /// `u128`.
    fn add(self, rhs: FinanceRational) -> Self::Output {
        let (n, d) = add_simple_rat(self.n, self.d, rhs.n, rhs.d)?;
        Ok(FinanceRational::new(n, d))
    }
}

impl Sub<FinanceRational> for FinanceRational {
    type Output = Result<Self, ArithmeticError>;
    /// Perform subtraction operation for two finance rational numbers.
    /// # Summary
    /// This function subtracts two finance rational numbers and returns the result.
    /// # Details
    /// The input rational numbers are subtracted from each other, and the result is reduced to its simplest form.
    /// # Errors
    /// This method may return `ArithmeticError::Overflow` if the result overflows `i128` or
    /// `u128`.
    fn sub(self, rhs: FinanceRational) -> Self::Output {
        self + (-rhs)?
    }
}

impl Neg for FinanceRational {
    type Output = Result<Self, ArithmeticError>;
    /// Perform negation for a finance rational number.
    /// # Summary
    /// This function negates a finance rational number.
    /// # Details
    /// The numerator of the rational number is negated, and the denominator is unchanged.
    /// # Errors
    /// This method may return `ArithmeticError::Overflow` if the result overflows `i128` or
    /// `u128`.
    fn neg(self) -> Self::Output {
        Ok(Self::new((-self.n)?, self.d))
    }
}

impl Mul for FinanceRational {
    type Output = Result<Self, ArithmeticError>;
    /// Perform multiplication operation for two finance rational numbers.
    /// # Summary
    /// This function multiplies two finance rational numbers and returns the result.
    /// # Details
    /// The input rational numbers are multiplied together, and the result is reduced to its simplest form.
    /// # Errors
    /// This method may return `ArithmeticError::Overflow` if the result overflows `i128` or
    /// `u128`.
    fn mul(self, rhs: FinanceRational) -> Self::Output {
        Ok(FinanceRational::new((self.n * rhs.n)?, (self.d * rhs.d)?))
    }
}

impl Div for FinanceRational {
    type Output = Result<Self, ArithmeticError>;
    /// Perform division operation for two finance rational numbers.
    /// # Summary
    /// This function divides two finance rational numbers and returns the result.
    /// # Details
    /// The input rational numbers are divided, and the result is reduced to its simplest form.
    /// # Errors
    /// This method may return `ArithmeticError::Overflow` if the result overflows `i128` or
    /// `u128`.
    #[allow(clippy::suspicious_arithmetic_impl)]
    fn div(self, rhs: FinanceRational) -> Self::Output {
        self * rhs.inv()? //Math is right. clippy will false positive.
    }
}

impl Inv for FinanceRational {
    type Output = Result<Self, ArithmeticError>;
    /// Perform reciprocal operation for a finance rational number.
    /// # Summary
    /// This function computes the reciprocal of a finance rational number.
    /// # Details
    /// The numerator and denominator of the rational number are swapped.
    /// # Errors
    /// This method may return `ArithmeticError::Overflow` if the result overflows `i128` or
    /// `u128`.
    fn inv(self) -> Self::Output {
        let neg = if self.n < FinanceInt(0) { true } else { false };
        let den = FinancePositive::try_from(self.n.0.abs())?;
        let num = FinanceInt::try_from(self.d.0.get())?;

        let nrat = FinanceRational::new(num, den);
        if neg { Ok((-nrat)?) } else { Ok(nrat) }
    }
}

impl From<FinanceInt> for FinanceRational {
    /// Create a rational number from an integer.
    /// # Summary
    /// This function converts a finance integer into a finance rational number.
    /// This value can be used in subsequent financial calculations.
    /// # Details
    /// The input integer is stored as the numerator of the rational number, and the denominator is set to 1.
    /// The greatest common divisor of the numerator and the denominator is 1.
    /// # Safety
    /// This method never fails, and the result is always a valid `FinanceRational`
    /// because no calculation is performed .
    fn from(value: FinanceInt) -> Self {
        Self::new(
            value,
            FinancePositive::try_from(1u128).expect("fixed value"),
        )
    }
}

fn add_simple_rat(
    n1: FinanceInt,
    d1: FinancePositive,
    n2: FinanceInt,
    d2: FinancePositive,
) -> Result<(FinanceInt, FinancePositive), ArithmeticError> {
    let cm = d1.clone() * d2.clone();
    let n = n1 * d2.try_into()?;
    let n2 = n2 * d1.try_into()?;
    let n_sum = n? + n2?;
    Ok((n_sum?, cm?))
}

#[cfg(test)]
mod test {
    use num::traits::Inv;

    use crate::finance_number::{integer::FinanceInt, positive::FinancePositive};

    use super::FinanceRational;
    use std::num::NonZeroU128;

    #[test]
    fn test_new() {
        let a = FinanceRational::new(FinanceInt(1), FinancePositive(NonZeroU128::new(2).unwrap()));
        assert_eq!(a.n, FinanceInt(1));
        assert_eq!(a.d, FinancePositive(NonZeroU128::new(2).unwrap()));
    }

    #[test]
    fn test_new_reduce_fraction() {
        let a = FinanceRational::new(FinanceInt(2), FinancePositive(NonZeroU128::new(4).unwrap()));
        assert_eq!(a.n, FinanceInt(1));
        assert_eq!(a.d, FinancePositive(NonZeroU128::new(2).unwrap()));
    }

    #[test]
    fn test_add() {
        let a = FinanceRational::new(FinanceInt(1), FinancePositive(NonZeroU128::new(2).unwrap()));
        let b = FinanceRational::new(FinanceInt(1), FinancePositive(NonZeroU128::new(2).unwrap()));
        let c = (a + b).expect("should not overflow");
        assert_eq!(
            c,
            FinanceRational::new(FinanceInt(1), FinancePositive(NonZeroU128::new(1).unwrap()))
        );
    }

    #[test]
    fn test_add_zero() {
        let a = FinanceRational::new(FinanceInt(1), FinancePositive(NonZeroU128::new(2).unwrap()));
        let b = FinanceRational::new(FinanceInt(0), FinancePositive(NonZeroU128::new(1).unwrap()));
        let c = (a + b).expect("should not overflow");
        assert_eq!(
            c,
            FinanceRational::new(FinanceInt(1), FinancePositive(NonZeroU128::new(2).unwrap()))
        );
    }

    #[test]
    fn test_add_negative() {
        let a = FinanceRational::new(FinanceInt(1), FinancePositive(NonZeroU128::new(2).unwrap()));
        let b = FinanceRational::new(
            FinanceInt(-1),
            FinancePositive(NonZeroU128::new(2).unwrap()),
        );
        let c = (a + b).expect("should not overflow");
        assert_eq!(
            c,
            FinanceRational::new(FinanceInt(0), FinancePositive(NonZeroU128::new(1).unwrap()))
        );
    }

    #[test]
    fn test_neg() {
        let a = FinanceRational::new(FinanceInt(1), FinancePositive(NonZeroU128::new(2).unwrap()));
        let b = (-a).expect("should not overflow");
        assert_eq!(
            b,
            FinanceRational::new(
                FinanceInt(-1),
                FinancePositive(NonZeroU128::new(2).unwrap())
            )
        );
    }

    #[test]
    fn test_sub() {
        let a = FinanceRational::new(FinanceInt(1), FinancePositive(NonZeroU128::new(2).unwrap()));
        let b = FinanceRational::new(FinanceInt(1), FinancePositive(NonZeroU128::new(3).unwrap()));
        let c = (a - b).expect("should not overflow");
        assert_eq!(
            c,
            FinanceRational::new(FinanceInt(1), FinancePositive(NonZeroU128::new(6).unwrap()))
        );
    }

    #[test]
    fn test_mul() {
        let a = FinanceRational::new(FinanceInt(1), FinancePositive(NonZeroU128::new(2).unwrap()));
        let b = FinanceRational::new(FinanceInt(3), FinancePositive(NonZeroU128::new(2).unwrap()));
        let c = (a * b).expect("should not overflow");
        assert_eq!(
            c,
            FinanceRational::new(FinanceInt(3), FinancePositive(NonZeroU128::new(4).unwrap()))
        );
    }

    #[test]
    fn test_mul_zero() {
        let a = FinanceRational::new(FinanceInt(1), FinancePositive(NonZeroU128::new(2).unwrap()));
        let b = FinanceRational::new(FinanceInt(0), FinancePositive(NonZeroU128::new(1).unwrap()));
        let c = (a * b).expect("should not overflow");
        assert_eq!(
            c,
            FinanceRational::new(FinanceInt(0), FinancePositive(NonZeroU128::new(1).unwrap()))
        );
    }

    #[test]
    fn test_inverse() {
        let a = FinanceRational::new(FinanceInt(1), FinancePositive(NonZeroU128::new(2).unwrap()));
        let b = a.inv().expect("should not overflow");
        assert_eq!(
            b,
            FinanceRational::new(FinanceInt(2), FinancePositive(NonZeroU128::new(1).unwrap()))
        );
    }

    #[test]
    fn test_neg_inverse() {
        let a = FinanceRational::new(
            FinanceInt(-1),
            FinancePositive(NonZeroU128::new(2).unwrap()),
        );
        let b = a.inv().expect("should not overflow");
        assert_eq!(
            b,
            FinanceRational::new(
                FinanceInt(-2),
                FinancePositive(NonZeroU128::new(1).unwrap())
            )
        );
    }

    #[test]
    fn test_div() {
        let a = FinanceRational::new(FinanceInt(1), FinancePositive(NonZeroU128::new(2).unwrap()));
        let b = FinanceRational::new(FinanceInt(1), FinancePositive(NonZeroU128::new(3).unwrap()));
        let c = (a / b).expect("should not overflow");
        assert_eq!(
            c,
            FinanceRational::new(FinanceInt(3), FinancePositive(NonZeroU128::new(2).unwrap()))
        );
    }
}
