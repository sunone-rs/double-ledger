use std::{
    marker::PhantomData,
    ops::{Add, Sub},
};

trait Currency {}

#[derive(Debug)]
struct Money<C>
where
    C: Currency,
{
    amount: u128,
    currency: PhantomData<C>,
}

impl<C> Add for Money<C>
where
    C: Currency,
{
    type Output = Money<C>;
    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            amount: self.amount + rhs.amount,
            currency: self.currency,
        }
    }
}

impl<C> Sub for Money<C>
where
    C: Currency,
{
    type Output = Money<C>;
    fn sub(self, rhs: Self) -> Money<C> {
        Self::Output {
            amount: self.amount - rhs.amount,
            currency: self.currency,
        }
    }
}
