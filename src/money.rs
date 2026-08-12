use std::{ops::Add, sync::Arc};

use num::CheckedAdd;

#[derive(Debug)]
pub struct Money<Currency>
where
    Currency: CurrencyUnit,
{
    amount: u128,
    rule: Arc<Currency>,
}

impl<T> Money<T> where T: CurrencyUnit {}

impl<T> CheckedAdd for Money<T>
where
    T: CurrencyUnit,
{
    fn checked_add(&self, v: &Self) -> Option<Self> {
        self.amount.checked_add(v.amount).map(|amount| Money {
            amount,
            rule: self.rule.clone(),
        })
    }
}

impl<T> Add for Money<T>
where
    T: CurrencyUnit,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        eprintln!(
            "\x1b[31m Unchecked add is used. \x1b[0m Use checked_add or MaybeCalc instead of this method."
        );
        Money {
            amount: self
                .amount
                .checked_add(rhs.amount)
                .expect("The Amount overflow."), // Safe guard for finance stability. Will be handled by MaybeCalc.
            rule: self.rule,
        }
    }
}

/// The trait about currency unit.
/// # Summary
/// This trait define properties and rules of currency.
///
pub trait CurrencyUnit {
    fn currency_code() -> String;
    fn currency_symbol() -> String;
    fn currency_decimal_places() -> u32;
}
