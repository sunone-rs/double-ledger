use std::fmt::Display;

use crate::finance_number::maybe::MaybeCalc;
use crate::money::*;
use chrono::{DateTime, Utc};

/// 貸借対照表の勘定科目の列挙
#[derive(PartialEq)]
pub enum AccountType {
    Asset,     // 資産
    Liability, // 負債
    Equity,    // 純資産
    Revenue,   // 収益
    Expense,   // 費用
}

/// 勘定科目
#[derive(PartialEq)]
pub struct Account {
    name: String,
    account_type: AccountType,
}

/// 取引
pub struct Transaction {
    timestamp:DateTime<Utc>,
    date_of_transaction:DateTime<Utc>,
    debit: Vec<(Account, Money)>,
    credit: Vec<(Account, Money)>,
}

pub enum TransactionError {
    Unbalanced,        //貸借不一致
    EmptyTransaction,  //空取引
    CalculationFailed, //計算などに失敗
}

impl Transaction {
    
    pub fn new(
        date_of_transaction: DateTime<Utc>,
        debit: Vec<(Account, Money)>,
        credit: Vec<(Account, Money)>,
    ) -> Result<Self, TransactionError> {
        let timestamp = Utc::now();
        if debit.is_empty() || credit.is_empty() {
            return Err(TransactionError::EmptyTransaction);
        }
        let (debit_sum, credit_sum): (MaybeCalc<Money>, MaybeCalc<Money>) = (
            debit.iter().map(|(_, m)| MaybeCalc::Just(*m)).sum(),
            credit.iter().map(|(_, m)| MaybeCalc::Just(*m)).sum(),
        );
        match debit_sum.and_then(|_| credit_sum) {
            MaybeCalc::Just(_) if debit_sum == credit_sum => Ok(Self { timestamp,date_of_transaction,debit, credit }),
            MaybeCalc::Just(_) => Err(TransactionError::Unbalanced),
            MaybeCalc::Error(_) => Err(TransactionError::CalculationFailed),
        }
    }

    /// 取引の規模
    /// # summary
    /// 取引の内容から規模を計算する。
    /// # returns
    /// 取引の規模を貨幣を用いて表す。
    pub fn volume(&self) -> Money{
        let total_debit:MaybeCalc<Money>  = self.debit.iter().map(|(_, m)| MaybeCalc::Just(*m)).sum();
        total_debit.expect("Success is ensured by `new` method.")
    }

    /// 科目を含むか
    /// # Summary 概要
    /// 取引の内容に特定の科目が含まれているか検査する。  
    /// 
    /// Test that contains specific account in this transaction.
    /// # Returns 戻り値
    /// 科目が含まれている場合は`true`そうでなければ`false`
    /// 
    /// If specific account has containd then `true` otherwise `false`.
    pub fn is_contain_account(&self, account: &Account) -> bool {
        self.debit.iter().chain(self.credit.iter()).any(|(acc,_)|acc==account)
    }

    /// 取引日の時刻を取得する
    /// # Summary 概要
    /// 取引の日時を取得する。
    /// 
    /// 取得される日時は記帳する際に取引の発生日として記載する事項である。
    /// 　　
    /// Retrive time of transaction.
    /// # Returns 戻り値
    /// 取引日時
    pub fn time_of_transaction(&self) -> DateTime<Utc> {
        self.date_of_transaction
    }

    /// 記録(登録)時刻を取得する
    /// # Summary 概要
    /// 記録した日時を取得する。
    /// 
    /// 記録した日時とは、このインスタンスが作成された時点とされます。
    /// 不変性によってそれ以降の変更がないことが保証されます。
    pub fn time_of_register(&self) -> DateTime<Utc> {
        self.timestamp
    }

}

///取引内容の表示
impl Display for Transaction{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "取引日時: {}", self.time_of_transaction())?;
        writeln!(f, "記録日時: {}", self.time_of_register())?;
        writeln!(f, "---------- 借方 (Debit) ----------")?;
        for (account, money) in &self.debit {
            writeln!(f, "  {} : {}", account.name, money)?;
        }
        writeln!(f, "---------- 貸方 (Credit) ----------")?;
        for (account, money) in &self.credit {
            writeln!(f, "  {} : {}", account.name, money)?;
        }
        writeln!(f, "合計: {}", self.volume())?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_transaction() {
        let jpy = register_currency("JPY").unwrap_or_else(|x| x.0);

        let credit = vec![(
            Account {
                name: "現金".to_string(),
                account_type: AccountType::Asset,
            },
            Money::new(1000, jpy),
        )];

        let debit = vec![(
            Account {
                name: "事業主借".to_string(),
                account_type: AccountType::Equity,
            },
            Money::new(1000, jpy),
        )];

        let transaction = Transaction::new(Utc::now(),debit, credit);
        assert!(transaction.is_ok());
    }

    #[test]
    fn test_unbalanced_transaction() {
        let jpy = register_currency("JPY").unwrap_or_else(|x| x.0);

        let credit = vec![(
            Account {
                name: "現金".to_string(),
                account_type: AccountType::Asset,
            },
            Money::new(1000, jpy),
        )];

        let debit = vec![(
            Account {
                name: "事業主借".to_string(),
                account_type: AccountType::Equity,
            },
            Money::new(900, jpy),
        )];

        let transaction = Transaction::new(Utc::now(),debit, credit);
        assert!(transaction.is_err());
    }

    #[test]
    fn test_currency_mismatched_transaction() {
        let jpy =register_currency("JPY").unwrap_or_else(|x| x.0);
        let usd=register_currency("USD").unwrap_or_else(|x|x.0);
        let credit = vec![(
            Account {
                name: "現金".to_string(),
                account_type: AccountType::Asset,
            },
            Money::new(1000, jpy),
        )];

        let debit = vec![(
            Account {
                name: "事業主借".to_string(),
                account_type: AccountType::Equity,
            },
            Money::new(1000, usd),
        )];

        let transaction = Transaction::new(Utc::now(),debit, credit);
        assert!(transaction.is_err());
    }



}

