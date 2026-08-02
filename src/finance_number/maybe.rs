use std::ops::{Add, Div, Mul, Rem, Sub};

use num::{CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, traits::CheckedRem};


/// 失敗するかもしれない計算を行うためのラップ型
/// Wrapping type that provide a calculation may be fail.
#[derive(Clone,Copy)]
pub struct MaybeCalc<T>(Option<T>);

impl<T> MaybeCalc<T>{
    pub fn just(from:T)->Self{
        MaybeCalc(Some(from))
    }

    pub fn unwrap(self)->T{
        self.0.unwrap()
    }
}

impl<T> From<Option<T>> for MaybeCalc<T>{
    fn from(value: Option<T>) -> Self {
        MaybeCalc(value)
    }
}

impl<T> From<T> for MaybeCalc<T>{
    fn from(value: T) -> Self {
        MaybeCalc(Some(value))
    }
}

impl<T> Add<Self> for MaybeCalc<T>
where T:CheckedAdd {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        MaybeCalc(self.0.and_then(|x|rhs.0.and_then(|y|x.checked_add(&y))))
    }
} 

impl<T> Sub<Self> for MaybeCalc<T>
where T:CheckedSub {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        MaybeCalc(self.0.and_then(|x|rhs.0.and_then(|y|x.checked_sub(&y))))
    }
}


impl<T> Mul<Self> for MaybeCalc<T>
where T:CheckedMul {
    type Output = Self;
    
    fn mul(self, rhs: Self) -> Self::Output {
        MaybeCalc(self.0.and_then(|x|rhs.0.and_then(|y|x.checked_mul(&y))))
    }
}

impl<T> Div<Self> for MaybeCalc<T>
where T:CheckedDiv {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        MaybeCalc(self.0.and_then(|x|rhs.0.and_then(|y|x.checked_div(&y))))
    }
}

impl<T> Rem<Self> for MaybeCalc<T>
where T:CheckedRem {
    type Output = Self;

    fn rem(self, rhs:Self) -> Self::Output {
        MaybeCalc(self.0.and_then(|x|rhs.0.and_then(|y|x.checked_rem(&y))))
    }
}




