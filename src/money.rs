use num::{CheckedAdd, Zero};
use parking_lot::RwLock;
use std::collections::HashMap;
use std::ops::Add;
use std::sync::LazyLock;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Money {
    amount: u128,
    rule: &'static Currency,
}
impl Money {
    pub fn new(amount: u128, rule: &'static Currency) -> Self {
        Self { amount, rule }
    }
}

impl Default for Money{
    fn default() -> Self {
        Self::zero()
    }
}
impl Zero for Money {
    fn zero() -> Self {
        let wild_zero_code = &"X00".to_string();
        let rule = register_currency(wild_zero_code).unwrap_or_else(|x| x.0);
        Self {
            amount: u128::zero(),
            rule,
        }
    }

    fn is_zero(&self) -> bool {
        self.amount == 0
    }
}
impl CheckedAdd for Money {
    fn checked_add(&self, rhs: &Self) -> Option<Self> {
        self.amount.checked_add(rhs.amount).and_then(|amount| {
            Some(Money {
                amount,
                rule: {
                    if self.rule == rhs.rule || rhs.rule.code.as_str() == "X00" {
                        self.rule
                    } else if self.rule.code.as_str() == "X00" {
                        rhs.rule
                    } else {
                        return None; //通貨違い無効
                    }
                },
            })
        })
    }
}
impl Add<Money> for Money {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        eprintln!(
            "\x1b[31m Unchecked add is used. \x1b[0m Use checked_add or MaybeCalc instead of this method."
        );

        Money {
            amount: self
                .amount
                .checked_add(rhs.amount)
                .expect("The Amount overflow."), // Safe guard for finance stability. Will be handled by MaybeCalc.
            rule: if self.rule == rhs.rule || rhs.rule.code == "X00" {
                self.rule
            } else if self.rule.code == "X00" {
                rhs.rule
            } else {
                panic!("Currency rule is not same.")
            },
        }
    }
}

impl std::fmt::Display for Money {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.amount, self.rule.code)
    }
}
pub fn get_currency_instance(code: &str) -> Option<&'static Currency> {
    //TODO implement registry search.

    let registry = GLOBAL_CURRENCY_REGISTRY.read();
    registry.get(code).copied()
}
pub fn register_currency(code: &str) -> Result<&'static Currency, CurrencyAlreadyRegistered> {
    let mut registry = GLOBAL_CURRENCY_REGISTRY.write();
    if !registry.contains_key(code) {
        let currency = Box::leak(Box::new(Currency {
            code: code.to_string(),
        }));
        registry.insert(code.to_string(), currency);
        return Ok(currency);
    }
    Err(CurrencyAlreadyRegistered(
        registry.get(code).expect("Record existance ensured. "),
    ))
}


#[derive(Debug)]
pub struct CurrencyAlreadyRegistered(pub &'static Currency);

/// The currency data.
/// # Summary
/// This trait define properties and rules of currency.

#[derive(Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct Currency {
    code: String,
}

static GLOBAL_CURRENCY_REGISTRY: LazyLock<RwLock<HashMap<String, &'static Currency>>> =
    LazyLock::new(|| RwLock::new(HashMap::<String, &'static Currency>::new()));

#[cfg(test)]
mod tests {}

