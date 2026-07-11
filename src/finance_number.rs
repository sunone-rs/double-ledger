//! Finance number module that provide safe calculation for finance.
//!
use std::error::Error;
use std::fmt::Display;

pub mod integer;
pub mod positive;
pub mod rational;

#[derive(Debug)]
pub enum ArithmeticError {
    Overflow,
    Div0,
    Indivisible(i128, i128),
}

impl std::error::Error for ArithmeticError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.source()
    }
}

impl Display for ArithmeticError {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!();
    }
}
