use crate::money::*;

/// 貸借対照表の勘定科目の列挙
pub enum AccountType {
    Asset,     // 資産
    Liability, // 負債
    Equity,    // 純資産
    Revenue,   // 収益
    Expense,   // 費用
}

/// 勘定科目
pub struct Account {
    name: String,
    account_type: AccountType,
}
