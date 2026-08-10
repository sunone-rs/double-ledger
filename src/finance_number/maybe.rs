use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

use num::{
    CheckedAdd, CheckedDiv, CheckedMul, CheckedSub,
    traits::{CheckedNeg, CheckedRem},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CalcErr {
    Overflow,
    Underflow,
    DivideByZero,
    NoneValue,
}

/// Wrapping type that provide a calculation may be fail.
///
/// # Safety
/// - panic not be occured before unwrap.
/// - checked arithmetic used.
///
#[derive(Clone, Copy)]
pub enum MaybeCalc<T> {
    Just(T),
    Error(CalcErr),
}

impl<T> MaybeCalc<T> {
    /// Create a MaybeCalc as a valid value.
    pub fn just(from: T) -> Self {
        MaybeCalc::Just(from)
    }

    /// Return the containd value and consuming self value.
    /// # Panics
    /// - When the MaybeCalc contains invalid value, panic is occured.
    pub fn unwrap(self) -> T {
        match self {
            MaybeCalc::Just(x) => x,
            MaybeCalc::Error(e) => panic!("Error value unwrapped {:?}", e),
        }
    }

    /// Return the Result<T,CalcError> and consuming self value.
    pub fn to_result(self) -> Result<T, CalcErr> {
        match self {
            MaybeCalc::Just(x) => Ok(x),
            MaybeCalc::Error(e) => Err(e),
        }
    }

    pub fn and_then<F, U>(self, f: F) -> MaybeCalc<U>
    where
        F: FnOnce(T) -> MaybeCalc<U>,
    {
        match self {
            MaybeCalc::Just(x) => f(x),
            MaybeCalc::Error(e) => MaybeCalc::Error(e),
        }
    }

    pub fn or_else<F>(self, f: F) -> MaybeCalc<T>
    where
        F: FnOnce(CalcErr) -> MaybeCalc<T>,
    {
        match self {
            MaybeCalc::Just(x) => MaybeCalc::Just(x),
            MaybeCalc::Error(e) => f(e),
        }
    }

    pub fn ok_or(self, err: CalcErr) -> Result<T, CalcErr> {
        match self {
            Self::Just(x) => Ok(x),
            Self::Error(_) => Err(err),
        }
    }
}

impl<T> From<Option<T>> for MaybeCalc<T> {
    fn from(value: Option<T>) -> Self {
        match value {
            Some(x) => MaybeCalc::Just(x),
            None => MaybeCalc::Error(CalcErr::NoneValue),
        }
    }
}

impl<T> From<T> for MaybeCalc<T> {
    fn from(value: T) -> Self {
        MaybeCalc::Just(value)
    }
}

impl<T> Add<Self> for MaybeCalc<T>
where
    T: CheckedAdd,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.and_then(|l| {
            rhs.and_then(|r| match l.checked_add(&r) {
                Some(x) => MaybeCalc::Just(x),
                None => MaybeCalc::Error(CalcErr::Overflow),
            })
        })
    }
}

impl<T> Sub<Self> for MaybeCalc<T>
where
    T: CheckedSub,
{
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        self.and_then(|l| {
            rhs.and_then(|r| match l.checked_sub(&r) {
                Some(x) => MaybeCalc::Just(x),
                None => MaybeCalc::Error(CalcErr::Overflow),
            })
        })
    }
}

impl<T> Mul<Self> for MaybeCalc<T>
where
    T: CheckedMul,
{
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        self.and_then(|l| {
            rhs.and_then(|r| match l.checked_mul(&r) {
                Some(x) => MaybeCalc::Just(x),
                None => MaybeCalc::Error(CalcErr::Overflow),
            })
        })
    }
}

impl<T> Div<Self> for MaybeCalc<T>
where
    T: CheckedDiv + PartialEq<T> + num::Zero,
{
    type Output = Self;
    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        self.and_then(|l| {
            rhs.and_then(|r| {
                if r == T::zero() {
                    return MaybeCalc::Error(CalcErr::DivideByZero);
                }
                match l.checked_div(&r) {
                    Some(x) => MaybeCalc::Just(x),
                    None => MaybeCalc::Error(CalcErr::Overflow),
                }
            })
        })
    }
}

impl<T> Rem<Self> for MaybeCalc<T>
where
    T: CheckedRem + PartialEq<T> + num::Zero,
{
    type Output = Self;
    #[inline]
    fn rem(self, rhs: Self) -> Self::Output {
        self.and_then(|l| {
            rhs.and_then(|r| {
                if r == T::zero() {
                    return MaybeCalc::Error(CalcErr::DivideByZero);
                }
                match l.checked_rem(&r) {
                    Some(x) => MaybeCalc::Just(x),
                    None => MaybeCalc::Error(CalcErr::Overflow),
                }
            })
        })
    }
}

impl<T> Neg for MaybeCalc<T>
where
    T: CheckedNeg,
{
    type Output = Self;
    #[inline]
    fn neg(self) -> Self::Output {
        self.and_then(|x| match x.checked_neg() {
            Some(x) => MaybeCalc::Just(x),
            None => MaybeCalc::Error(CalcErr::Overflow),
        })
    }
}

#[cfg(test)]
mod test {
    use crate::finance_number::maybe::CalcErr;

    use super::MaybeCalc;

    #[test]
    fn test_add() {
        let a = MaybeCalc::just(1);
        let b = MaybeCalc::just(2);
        let c = a + b;
        assert_eq!(c.unwrap(), 3);
    }

    #[test]
    #[should_panic]
    fn test_bad_add_panic() {
        let a = MaybeCalc::just(i128::MAX);
        let b = MaybeCalc::just(1);
        let c = a + b;
        c.unwrap();
    }

    #[test]
    fn test_bad_add_return_overflow() {
        let a = MaybeCalc::just(i128::MAX);
        let b = MaybeCalc::just(1);
        let c = a + b;
        assert!(matches!(c, MaybeCalc::Error(CalcErr::Overflow)));
    }

    #[test]
    fn test_sub() {
        let a = MaybeCalc::just(1);
        let b = MaybeCalc::just(2);
        let c = a - b;
        assert_eq!(c.unwrap(), -1);
    }

    #[test]
    fn test_bad_sub_return_none() {
        let a = MaybeCalc::just(i128::MIN);
        let b = MaybeCalc::just(1);
        let c = a - b;
        assert!(matches!(c, MaybeCalc::Error(CalcErr::Overflow)));
    }

    #[test]
    #[should_panic]
    fn test_bad_sub_panic() {
        let a = MaybeCalc::just(i128::MIN);
        let b = MaybeCalc::just(1);
        let c = a - b;
        c.unwrap();
    }

    #[test]
    fn test_mul() {
        let a = MaybeCalc::just(1);
        let b = MaybeCalc::just(2);
        let c = a * b;
        assert_eq!(c.unwrap(), 2);
    }

    #[test]
    #[should_panic]
    fn test_bad_mul_panic() {
        let a = MaybeCalc::just(i128::MAX);
        let b = MaybeCalc::just(2);
        let c = a * b;
        c.unwrap();
    }

    #[test]
    fn test_bad_mul_return_none() {
        let a = MaybeCalc::just(i128::MAX);
        let b = MaybeCalc::just(2);
        let c = a * b;
        assert!(matches!(c, MaybeCalc::Error(CalcErr::Overflow)));
    }

    #[test]
    fn test_div() {
        let a = MaybeCalc::just(2);
        let b = MaybeCalc::just(2);
        let c = a / b;
        assert_eq!(c.unwrap(), 1);
    }

    #[test]
    fn test_div_undivisible() {
        let a = MaybeCalc::just(1);
        let b = MaybeCalc::just(2);
        let c = a / b;
        assert_eq!(c.unwrap(), 0);
    }

    #[test]
    fn test_div_by_zero() {
        let a = MaybeCalc::just(i128::MAX);
        let b = MaybeCalc::just(0);
        let c = a / b;
        assert!(matches!(c, MaybeCalc::Error(CalcErr::DivideByZero)));
    }

    #[test]
    #[should_panic]
    fn test_bad_div_panic() {
        let a = MaybeCalc::just(i128::MAX);
        let b = MaybeCalc::just(0);
        let c = a / b;
        c.unwrap();
    }

    #[test]
    fn test_rem() {
        let a = MaybeCalc::just(1);
        let b = MaybeCalc::just(2);
        let c = a % b;
        assert_eq!(c.unwrap(), 1);
    }

    #[test]
    fn test_rem_by_zero() {
        let a = MaybeCalc::just(i128::MAX);
        let b = MaybeCalc::just(0);
        let c = a % b;
        assert!(matches!(c, MaybeCalc::Error(CalcErr::DivideByZero)));
    }

    #[test]
    #[should_panic]
    fn test_bad_rem_panic() {
        let a = MaybeCalc::just(i128::MAX);
        let b = MaybeCalc::just(0);
        let c = a % b;
        c.unwrap();
    }

    #[test]
    fn test_div_min_neg_one() {
        let a = MaybeCalc::just(i128::MIN);
        let b = MaybeCalc::just(-1);
        let c = a / b;
        assert!(matches!(c, MaybeCalc::Error(CalcErr::Overflow)));
    }

    #[test]
    fn test_adversarial_rem_min_neg_one() {
        let a = MaybeCalc::just(i128::MIN);
        let b = MaybeCalc::just(-1);
        let c = a % b;
        assert!(matches!(c, MaybeCalc::Error(CalcErr::Overflow)));
    }

    #[test]
    fn test_adversarial_neg_min() {
        let a = MaybeCalc::just(i128::MIN);
        let c = -a;
        assert!(matches!(c, MaybeCalc::Error(CalcErr::Overflow)));
    }

    #[test]
    fn test_adversarial_none_propagation() {
        let none = MaybeCalc::<i32>::from(None);
        let some = MaybeCalc::just(10);

        // None + Some = None
        assert!(matches!(none + some, MaybeCalc::Error(CalcErr::NoneValue)));

        // Some + None = None
        assert!(matches!(some + none, MaybeCalc::Error(CalcErr::NoneValue)));

        // None + None = None
        assert!(matches!(none + none, MaybeCalc::Error(CalcErr::NoneValue)));
    }
}
