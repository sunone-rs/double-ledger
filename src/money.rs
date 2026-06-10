
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
    type Output = MoneyResult;
    fn add(self,other:Money)->MoneyResult{
        if self.currency == other.currency{
            MoneyResult(Ok(Money::new(self.amount+other.amount,self.currency)))
        }else{
            MoneyResult(Err(CurrencyNotMatch(self.currency,other.currency)));
        }
    }
}
impl Add<MoneyResult> for Money{
    type Output = MoneyResult;
    fn Add(self,other:MoneyResult)->MoneyResult{
        if let Ok(money) = other{
            self.add(money)
        }else{
            Propergation(Ok(self),other)
        }
    }

}
impl Sub<Money> for Money{
    type Output = MoneyResult;
    fn sub(self,other:Money)->MoneyResult{
        if self.currency == other.currency{
            MoneyResult(Ok(Money::new(self.amount-other.amount,self.currency)))
        }else{
            MoneyResult(Err(CurrencyNotMatch(self.currency,other.currency)));
        }
    }
}

impl Sub<MoneyResult> for Money{
    type Output = MoneyResult;
    fn sub(self,other:MoneyResult)->MoneyResult{
        if let Ok(money) = other{
            self.sub(money)
        }else{
            MoneyResult(Propergation(MoneyResult(Ok(Self)),other))
        }
    }
}



pub enum MoneyError{
    CurrencyNotMatch(MoneyResult, MoneyResult),
    Propergation(MoneyResult,MoneyResult)
}

mod money_result{
    use super::*;
    use std::ops::{Add,Sub};


    pub struct MoneyResult(Result<Money,MoneyError>);

    impl Add<Money> for MoneyResult{
        type Output = MoneyResult;
        fn add(self,other:Money)->MoneyResult{
            
            if let Ok(money) = self {
                money + other
            }else{
                Propergation(self,other)
            }
        }
    }


    impl Add<MoneyResult> for MoneyResult{
        type Output = MoneyResult;
        fn add(self,other:MoneyResult)->MoneyResult{
            if let (Ok(money),Ok(other)) = (self,other) {
                money + other
            } else {
                MoneyResult(Propergation(self,other))
            }
        }
    }


    impl Sub<Money> for MoneyResult{
        type Output = MoneyResult;
        fn sub(self,other:Money)->MoneyResult{
            if let Ok(money) = self {
                money - other
            } else {
                MoneyResult(Propergation(self,other))
            }
        }
    }


    impl Sub<MoneyResult> for MoneyResult{
        type Output = MoneyResult;
        fn sub(self,other:MoneyResult)->MoneyResult{
            if let (Ok(money),Ok(other)) =(self,other){
                money-other
            } else {
                MoneyResult(Propergation (self,other))
            }
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