use std::{num::NonZeroU128, ops::Add};

use num_integer::gcd;

use crate::finance_number::ArithmeticError;

use super::integer::FinanceInt;
use super::positive::FinancePositive;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct FinanceRational {
    n: FinanceInt,
    d: FinancePositive,
}

impl FinanceRational {
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

    fn add(self, rhs: FinanceRational) -> Self::Output {
        let (n, d) = internal::add_simple_rat(self.n, self.d, rhs.n, rhs.d)?;
        Ok(FinanceRational::new(n, d))
    }
}

mod internal {
    use super::ArithmeticError;
    use super::FinanceInt;
    use super::FinancePositive;

    pub fn add_simple_rat(
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
}

#[cfg(test)]
mod test {
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
}
