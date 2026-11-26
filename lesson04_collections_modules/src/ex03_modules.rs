//! 练习 3：模块封装、可见性（`pub` / `pub(super)`）与 `pub use` 重导出
//!
//! 场景：我们有一个银行账户模块 `bank`，账户余额 `balance` 必须是私有的，
//! 外部不能直接篡改 `account.balance = 999999`，只能通过公开方法存取款。

pub mod bank {
    #[derive(Debug, PartialEq)]
    pub enum BankError {
        InvalidAmount,
        InsufficientFunds,
    }

    #[derive(Debug)]
    pub struct Account {
        pub owner: String,
        balance: u64, // 私有字段，仅 bank 模块内部可访问
    }

    impl Account {
        /// 3.1 创建一个初始余额为 0 的账户。
        pub fn new(owner: &str) -> Self {
            Account {
                owner: owner.to_string(),
                balance: 0,
            }
        }

        /// 3.2 查询当前余额（只读）。
        pub fn balance(&self) -> u64 {
            self.balance
        }

        /// 3.3 存款：如果 `amount == 0` 返回 `Err(BankError::InvalidAmount)`，否则累加余额并返回 `Ok(())`。
        pub fn deposit(&mut self, amount: u64) -> Result<(), BankError> {
            match amount {
                0 => Err(BankError::InvalidAmount),
                _ => {
                    self.balance += amount;
                    Ok(())
                }
            }
        }

        /// 3.4 取款：
        /// - `amount == 0` -> `Err(BankError::InvalidAmount)`
        /// - `amount > self.balance` -> `Err(BankError::InsufficientFunds)`
        /// - 否则扣减余额并返回 `Ok(())`
        pub fn withdraw(&mut self, amount: u64) -> Result<(), BankError> {
            match amount {
                0 => Err(BankError::InvalidAmount),
                _ => {
                    if self.balance < amount {
                        Err(BankError::InsufficientFunds)
                    } else {
                        self.balance -= amount;
                        Ok(())
                    }
                }
            }
        }
    }
}

/// 3.5 重导出（re-export）：让调用方可以直接用 `ex03_modules::Account` 和 `ex03_modules::BankError`，
/// 而不必每次都写 `ex03_modules::bank::Account`。
pub use bank::{Account, BankError};

#[cfg(test)]
mod tests {
    use super::{Account, BankError};

    #[test]
    fn test_account_flow() {
        let mut acc = Account::new("Jesse");
        assert_eq!(acc.owner, "Jesse");
        assert_eq!(acc.balance(), 0);

        assert_eq!(acc.deposit(0), Err(BankError::InvalidAmount));
        assert_eq!(acc.deposit(500), Ok(()));
        assert_eq!(acc.balance(), 500);

        assert_eq!(acc.withdraw(600), Err(BankError::InsufficientFunds));
        assert_eq!(acc.withdraw(200), Ok(()));
        assert_eq!(acc.balance(), 300);
    }
}
