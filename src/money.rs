use std::{
    marker::PhantomData,
    ops::{Add, Sub},
};

use crate::finance_number::FinanceInt;
pub trait Currency {}

#[derive(Debug)]
pub struct Money<C>
where
    C: Currency,
{
    _amount: FinanceInt,
    currency: PhantomData<C>,
}

impl<C> Add for Money<C>
where
    C: Currency,
{
    type Output = Money<C>;
    fn add(self, _rhs: Self) -> Self::Output {
        todo!("Impl add operation")
    }
}

impl<C> Sub for Money<C>
where
    C: Currency,
{
    type Output = Money<C>;
    fn sub(self, _rhs: Self) -> Money<C> {
        todo!("Impl sub operation")
    }
}
