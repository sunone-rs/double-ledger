use std::{
    marker::PhantomData,
    ops::{Add, Sub},
};

use crate::finance_number::integer::FinanceInt;
pub trait Currency {}

#[derive(Debug)]
pub struct Money{
    amount:FinanceInt,
}