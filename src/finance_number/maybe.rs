// Copyright (C) 2026 SuNone_rs
// License:Apache2.0 or MIT

//! The MaybeCalc is a wrapper type that provides arithmetic operations that may fail.
//! It is used to prevent crash or data pollution caused by overflow,underflow,and some arithmetic errors.
//! # Safety
//! - panic not be occured before unwrap.
//! - checked arithmetic used.
//! - If error, the MaybeCalc will return the Error variant and do not panic.
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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MaybeCalc<T> {
    Just(T),
    Error(CalcErr),
}

impl<T> MaybeCalc<T> {
    /// Create a MaybeCalc as a valid value.
    #[inline]
    pub fn just(from: T) -> Self {
        MaybeCalc::Just(from)
    }

    /// Return the containd value and consuming self value.
    /// # Panics
    /// - When the MaybeCalc contains invalid value, panic is occured.
    #[inline]
    pub fn unwrap(self) -> T {
        match self {
            MaybeCalc::Just(x) => x,
            MaybeCalc::Error(e) => panic!("Error value unwrapped {:?}", e),
        }
    }

    /// Return the containd value and consuming self value.
    /// # Panics
    /// - When the MaybeCalc contains invalid value, panic is occured.
    #[inline]
    pub fn expect(self, msg: &str) -> T {
        match self {
            MaybeCalc::Just(x) => x,
            MaybeCalc::Error(_) => panic!("{}", msg),
        }
    }

    /// Return if the MaybeCalc contains a value.
    /// # Returns
    /// - `true` if the MaybeCalc contains a value.
    /// - `false` if the MaybeCalc contains an error.
    ///
    /// # Example
    /// ```rust
    /// use double_ledger::finance_number::maybe::*;
    ///
    /// let a = MaybeCalc::Just(1);
    /// assert_eq!(a.is_just(), true);
    ///
    /// let b: MaybeCalc<i32> = MaybeCalc::Error(CalcErr::Overflow);
    /// assert_eq!(b.is_just(), false);
    /// ```
    #[inline]
    pub fn is_just(self) -> bool {
        match self {
            MaybeCalc::Just(_) => true,
            MaybeCalc::Error(_) => false,
        }
    }

    /// Return if the MaybeCalc contains an error.
    /// # Returns
    /// - `true` if the MaybeCalc contains an error.
    /// - `false` if the MaybeCalc contains a value.
    ///
    /// # Example
    /// ```rust
    /// use double_ledger::finance_number::maybe::MaybeCalc;
    /// use double_ledger::finance_number::maybe::CalcErr;
    ///
    /// let a = MaybeCalc::Just(1);
    /// assert_eq!(a.is_err(), false);
    ///
    /// let b:MaybeCalc<i32> = MaybeCalc::Error(CalcErr::Overflow);
    /// assert_eq!(b.is_err(), true);
    /// ```
    #[inline]
    pub fn is_err(self) -> bool {
        match self {
            MaybeCalc::Just(_) => false,
            MaybeCalc::Error(_) => true,
        }
    }

    /// Convert to the std Result.
    /// It allow to use to ? operator for early return.
    /// # Returns
    /// - `Ok(T)` if the MaybeCalc contains a value.
    /// - `Err(CalcErr)` if the MaybeCalc contains an error.
    ///
    /// # Example1
    /// ```rust
    /// use double_ledger::finance_number::maybe::MaybeCalc;
    /// use double_ledger::finance_number::maybe::CalcErr;
    ///
    /// let a = MaybeCalc::Just(1);
    /// assert_eq!(a.to_result(), Ok(1));
    ///
    /// let b:MaybeCalc<i32>= MaybeCalc::Error(CalcErr::Overflow);
    /// assert_eq!(b.to_result(), Err(CalcErr::Overflow));
    /// ```
    #[inline]
    pub fn to_result(self) -> Result<T, CalcErr> {
        match self {
            MaybeCalc::Just(x) => Ok(x),
            MaybeCalc::Error(e) => Err(e),
        }
    }

    /// Call `op` if `self` is `Just`.
    /// If `self` is `Error`, it will no action and return `Error`.
    /// original object be consumed because type translation.
    /// # Arguments
    /// * `op` - The function to call if `self` is `Just`.
    /// # Returns
    /// - `op(x)` if `self` is `Just(x)`.
    /// - `Err(e)` if `self` is `Error(e)`.
    ///
    /// # Example
    /// ```rust
    /// use double_ledger::finance_number::maybe::MaybeCalc;
    /// use double_ledger::finance_number::maybe::CalcErr;
    ///
    /// let a = MaybeCalc::Just(1);
    /// let b = a.and_then(|x| MaybeCalc::Just(x + 1));
    /// assert_eq!(b.unwrap(), 2);
    ///
    /// let c:MaybeCalc<i32>= MaybeCalc::Error(CalcErr::Overflow);
    /// let d = c.and_then(|x| MaybeCalc::Just(x + 1));
    /// assert_eq!(d.unwrap_or(0), 0);
    /// ```
    #[inline]
    pub fn and_then<F, U>(self, op: F) -> MaybeCalc<U>
    where
        F: FnOnce(T) -> MaybeCalc<U>,
    {
        match self {
            MaybeCalc::Just(x) => op(x),
            MaybeCalc::Error(e) => MaybeCalc::Error(e),
        }
    }

    /// Call `op` if `self` is `Error`.
    /// If `self` is `Just`, it will no action and return `Just`.
    /// original object be consumed because type translation.
    /// It is used for handling and recover error.
    /// # Arguments
    /// * `op` - The function to call if `self` is `Error`.
    /// # Returns
    /// - `op(e)` if `self` is `Error(e)`.
    /// - `Just(x)` if `self` is `Just(x)`.
    /// # Example
    /// ```rust
    /// use double_ledger::finance_number::maybe::MaybeCalc;
    /// use double_ledger::finance_number::maybe::CalcErr;
    ///
    /// let a = MaybeCalc::Just(1);
    /// let b = a.or_else(|_| MaybeCalc::Just(0));
    /// assert_eq!(b.unwrap(), 1);
    ///
    /// let c:MaybeCalc<i32>= MaybeCalc::Error(CalcErr::Overflow);
    /// let d = c.or_else(|_| MaybeCalc::Just(1));
    /// assert_eq!(d.unwrap(), 1);
    /// ```
    #[inline]
    pub fn or_else<F>(self, f: F) -> MaybeCalc<T>
    where
        F: FnOnce(CalcErr) -> MaybeCalc<T>,
    {
        match self {
            MaybeCalc::Just(x) => MaybeCalc::Just(x),
            MaybeCalc::Error(e) => f(e),
        }
    }

    /// Return default value if `self` is `Error`.
    /// If `self` is `Just`, it will no action and return `Just`.
    /// original object be consumed because type translation.
    /// # Arguments
    /// * `v` - The value to return if `self` is `Error`.
    /// # Returns
    /// - `Just(x)` if `self` is `Just(x)`.
    /// - `Just(v)` if `self` is `Error(e)`.
    #[inline]
    pub fn or_default(self) -> MaybeCalc<T>
    where
        T: Default,
    {
        match self {
            MaybeCalc::Just(x) => MaybeCalc::Just(x),
            MaybeCalc::Error(_) => MaybeCalc::Just(Default::default()),
        }
    }

    /// Return value of `self` if `self` is `Just`. Otherwise return `v`.
    /// original object be consumed.
    /// # Arguments
    /// * `v` - The value to return if `self` is `Error`.
    /// # Returns
    /// - `Just(x)` if `self` is `Just(x)`.
    /// - `Just(v)` if `self` is `Error(e)`.
    #[inline]
    pub fn unwrap_or(self, v: T) -> T {
        match self {
            MaybeCalc::Just(x) => x,
            MaybeCalc::Error(_) => v,
        }
    }

    /// Return value of `self` if `self` is `Just`. Otherwise return `op(e)`.
    /// original object be consumed.
    /// # Arguments
    /// * `op` - The function to call if `self` is `Error`.
    /// # Returns
    /// - `Just(x)` if `self` is `Just(x)`.
    /// - `op(e)` if `self` is `Error(e)`.
    #[inline]
    pub fn unwrap_or_else<F>(self, op: F) -> T
    where
        F: FnOnce(CalcErr) -> T,
    {
        match self {
            MaybeCalc::Just(x) => x,
            MaybeCalc::Error(e) => op(e),
        }
    }

    /// Convert to `Result`.
    /// If `self` is `Just`, it will return `Ok(x)`.
    /// If `self` is `Error`, it will return `Err(err)`.
    /// original object be consumed.
    /// # Arguments
    /// * `err` - The error to return if `self` is `Error`.
    /// # Returns
    /// - `Ok(x)` if `self` is `Just(x)`.
    /// - `Err(err)` if `self` is `Error(e)`.
    /// # Example
    /// ```rust
    /// use double_ledger::finance_number::maybe::MaybeCalc;
    /// use double_ledger::finance_number::maybe::CalcErr;
    ///
    /// let a = MaybeCalc::just(1);
    /// assert_eq!(a.ok_or(CalcErr::Overflow), Ok(1));
    ///
    /// let b: MaybeCalc<i32> = MaybeCalc::Error(CalcErr::Overflow);
    /// assert_eq!(b.ok_or(CalcErr::Overflow), Err(CalcErr::Overflow));
    /// ```
    #[inline]
    pub fn ok_or(self, err: CalcErr) -> Result<T, CalcErr> {
        match self {
            Self::Just(x) => Ok(x),
            Self::Error(_) => Err(err),
        }
    }
}

impl<T> From<Option<T>> for MaybeCalc<T> {
    /// Convert `Option<T>` to `MaybeCalc<T>`.
    /// original object be consumed.
    /// # Arguments
    /// * `value` - The value to convert.
    /// # Returns
    /// - `MaybeCalc::Just(x)` if `self` is `Some(x)`.
    /// - `MaybeCalc::Error(CalcErr::NoneValue)` if `self` is `None`.
    /// # Example
    /// ```rust
    /// use double_ledger::finance_number::maybe::MaybeCalc;
    /// use double_ledger::finance_number::maybe::CalcErr;
    ///
    /// let a:Option<i32> = Some(1);
    /// assert_eq!(MaybeCalc::from(a), MaybeCalc::Just(1));
    ///
    /// let b:Option<i32> = None;//if error value you may need to add type annotation to make it compile
    /// assert_eq!(MaybeCalc::<i32>::from(b), MaybeCalc::Error(CalcErr::NoneValue));
    /// ```
    #[inline]
    fn from(value: Option<T>) -> Self {
        match value {
            Some(x) => MaybeCalc::Just(x),
            None => MaybeCalc::Error(CalcErr::NoneValue),
        }
    }
}

/// Convert numbers into Just
/// # Example
/// ```rust
/// use double_ledger::finance_number::maybe::MaybeCalc;
///
/// let a = MaybeCalc::from(1);
///
/// ```
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

    ///Safety add method for MaybeCalc
    ///
    /// # Arguments
    /// * `self` - The first number.
    /// * `rhs` - The second number.
    /// # Returns
    /// - `MaybeCalc::Just(x)` if `self` and `rhs` are `Just` and result not overflowed.
    /// - `MaybeCalc::Error(CalcErr::Overflow)` if `self` or `rhs` is `Error` or result was overflowed.
    /// # Example
    /// ```rust
    /// use double_ledger::finance_number::maybe::MaybeCalc;
    /// use double_ledger::finance_number::maybe::CalcErr;
    /// use std::ops::Add;
    ///
    /// let a = MaybeCalc::just(1);
    /// let b = MaybeCalc::just(2);
    /// assert_eq!(a.add(b), MaybeCalc::Just(3));
    ///
    /// let c:MaybeCalc<i32>= MaybeCalc::Error(CalcErr::Overflow);
    /// assert_eq!(c.add(b), MaybeCalc::<i32>::Error(CalcErr::Overflow));
    /// ```
    #[inline]
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
    ///Safety sub method for MaybeCalc
    /// # Arguments
    /// * `self` - The first number.
    /// * `rhs` - The second number.
    /// # Returns
    /// - `MaybeCalc::Just(x)` if `self` and `rhs` are `Just` and result not overflowed.
    /// - `MaybeCalc::Error(CalcErr::Overflow)` if `self` or `rhs` is `Error` or result was overflowed.
    /// # Example
    /// ```rust
    /// use double_ledger::finance_number::maybe::*;
    /// use std::ops::Sub;
    ///
    /// let a = MaybeCalc::just(1);
    /// let b = MaybeCalc::just(2);
    /// assert_eq!(a.sub(b), MaybeCalc::Just(-1));
    ///
    /// let c:MaybeCalc<i32>= MaybeCalc::Error(CalcErr::Overflow);
    /// assert_eq!(c.sub(b), MaybeCalc::Error(CalcErr::Overflow));
    /// ```
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
    /// Safety mul operation
    /// # Arguments
    /// * `self` - The left hand of this operation.
    /// * `rhs` - The right hand of this operation.
    /// # Returns
    /// - `MaybeCalc::Just(x)` if `self` and `rhs` are `Just` and result not overflowed.
    /// - `MaybeCalc::Error(CalcErr::Overflow)` if `self` or `rhs` is `Error` or result was overflowed.
    /// # Example
    /// ```rust
    /// use double_ledger::finance_number::maybe::*;
    /// use std::ops::Mul;
    ///
    /// let a = MaybeCalc::just(1);
    /// let b = MaybeCalc::just(2);
    /// assert_eq!(a.mul(b), MaybeCalc::Just(2));
    ///
    /// let c:MaybeCalc<i32>= MaybeCalc::Error(CalcErr::Overflow);
    /// assert_eq!(c.mul(b), MaybeCalc::<i32>::Error(CalcErr::Overflow));
    /// ```
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
    /// Safe divide method for MaybeCalc
    /// # Arguments
    /// * `self` - The first number.
    /// * `rhs` - The second number.
    /// # Returns
    /// - `MaybeCalc::Just(x)` if `self` and `rhs` are `Just` and result not overflowed or Divide by zero.
    /// - `MaybeCalc::Error(CalcErr::Overflow)` if `self` or `rhs` is `Error` or result was overflowed.
    /// - `MaybeCalc::Error(CalcErr::DivideByZero)` if `rhs` is zero.
    /// # Example
    /// ```rust
    /// use double_ledger::finance_number::maybe::MaybeCalc;
    /// use double_ledger::finance_number::maybe::CalcErr;
    /// use std::ops::Div;
    ///
    /// let a = MaybeCalc::Just(1);
    /// let b = MaybeCalc::Just(2);
    /// assert_eq!(a.div(b), MaybeCalc::Just(1/2));
    ///
    /// let c = MaybeCalc::Error(CalcErr::Overflow);
    /// assert_eq!(c.div(b), MaybeCalc::Error(CalcErr::Overflow));
    /// ```
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

    /// Safe remainder method for MaybeCalc
    /// # Arguments
    /// * `self` - The first number.
    /// * `rhs` - The second number.
    /// # Returns
    /// - `MaybeCalc::Just(x)` if `self` and `rhs` are `Just` and result not overflowed or Divide by zero.
    /// - `MaybeCalc::Error(CalcErr::Overflow)` if `self` or `rhs` is `Error` or result was overflowed.
    /// - `MaybeCalc::Error(CalcErr::DivideByZero)` if `rhs` is zero.
    /// # Example
    /// ```rust
    /// use double_ledger::finance_number::maybe::*;
    /// use std::ops::Rem;
    ///
    /// let a = MaybeCalc::Just(1);
    /// let b = MaybeCalc::Just(2);
    /// assert_eq!(a.rem(b), MaybeCalc::Just(1%2));
    ///
    /// let c:MaybeCalc<i32> = MaybeCalc::Error(CalcErr::Overflow);
    /// assert_eq!(c.rem(b), MaybeCalc::<i32>::Error(CalcErr::Overflow));
    /// ```
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

    /// Safety Neg operation
    /// # Arguments
    /// * `self` - The first number.
    /// # Returns
    /// - `MaybeCalc::Just(x)` if `self` is `Just` and result not overflowed.
    /// - `MaybeCalc::Error(CalcErr::Overflow)` if `self` is `Error` or result was overflowed.
    /// # Example
    /// ```rust
    /// use double_ledger::finance_number::maybe::*;
    /// use std::ops::Neg;
    ///
    /// let a = MaybeCalc::just(1);
    /// assert_eq!(a.neg(), MaybeCalc::Just(-1));
    /// ```
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
