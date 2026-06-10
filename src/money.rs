
use std::ops::{Add,Sub};
use MoneyError::{CurrencyNotMatch};
use money_result::MoneyResult;



pub struct Money{
    amount:i128,
    currency:String,
}

impl Money{
    pub fn new(amount:i128,currency:String)->Money{
        Money{amount,currency}
    }
}

impl Add<Money> for Money{
    type Output = money_result::MoneyResult;
    fn add(self,other:Money)->money_result::MoneyResult{
        if self.currency == other.currency{
            money_result::MoneyResult(Ok(Money::new(self.amount+other.amount,self.currency)))
        }else{
            money_result::MoneyResult(Err(CurrencyNotMatch(self.currency,other.currency)));
        }
    }
}
impl Add<money_result::MoneyResult> for Money{
    type Output = money_result::MoneyResult;
    fn Add(self,other:money_result::MoneyResult)->money_result::MoneyResult{
        if let Ok(money) = other{
            self.add(money)
        }else{
            other
        }
    }

}
impl Sub<Money> for Money{
    type Output = money_result::MoneyResult;
    fn sub(self,other:Money)->money_result::MoneyResult{
        if self.currency == other.currency{
            money_result::MoneyResult(Ok(Money::new(self.amount-other.amount,self.currency)))
        }else{
            money_result::MoneyResult(Err(CurrencyNotMatch(self.currency,other.currency)));
        }
    }
}

impl Sub<money_result::MoneyResult> for Money{
    type Output = money_result::MoneyResult;
    fn sub(self,other:money_result::MoneyResult)->money_result::MoneyResult{
        if let Ok(money) = other{
            self.sub(money)
        }else{
            other
        }
    }
}



pub enum MoneyError{
    CurrencyNotMatch(String,String),
}

mod money_result{
    use super::*;
    use std::ops::{Add,Sub};


    pub struct MoneyResult(Result<Money,MoneyError>);

    impl Add<Money> for MoneyResult{
        type Output = MoneyResult;
        fn add(self,other:Money)->MoneyResult{
            MoneyResult(self.0.add(other))
        }
    }
    impl Add<MoneyResult> for MoneyResult{
        type Output = MoneyResult;
        fn add(self,other:MoneyResult)->MoneyResult{
            MoneyResult(self.0.add(other.0))
        }
    }
    impl Sub<Money> for MoneyResult{
        type Output = MoneyResult;
        fn sub(self,other:Money)->MoneyResult{
            MoneyResult(self.0.sub(other))
        }
    }
    impl Sub<MoneyResult> for MoneyResult{
        type Output = MoneyResult;
        fn sub(self,other:MoneyResult)->MoneyResult{
            MoneyResult(self.0.sub(other.0))
        }
    }

}

#[cfg(test)]
mod tests{
    #[test]
    fn add_money_test(){
        let result = Money::new(100,"JPY")+Money::new(200,"JPY");
        assert_eq!(result.unwrap(),Money::new(300,"JPY"));
    }

}