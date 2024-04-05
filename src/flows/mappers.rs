use crate::{
    accounts_integration::AccountsIntegrationOperationResult,
    accounts_manager_grpc::AccountsManagerOperationResult,
};

use crate::{accounts_integration::*, accounts_manager_grpc::*};

pub fn convert_result<TResult>(
    op_result: AccountsManagerOperationResult,
    get_result: impl FnOnce() -> TResult,
) -> Result<TResult, AccountsIntegrationOperationResult> {
    match op_result {
        AccountsManagerOperationResult::Ok => Ok(get_result()),
        AccountsManagerOperationResult::AccountNotFound => {
            Err(AccountsIntegrationOperationResult::AccountNotFound)
        }
        AccountsManagerOperationResult::TraderNotFound => {
            Err(AccountsIntegrationOperationResult::TraderNotFound)
        }
        AccountsManagerOperationResult::NotEnoughBalance => {
            Err(AccountsIntegrationOperationResult::NotEnoughBalance)
        }
        AccountsManagerOperationResult::ProcessIdDuplicate => {
            Err(AccountsIntegrationOperationResult::ProcessIdDuplicate)
        }
    }
}

impl Into<AccountsIntegrationClientAccountGrpcModel> for AccountGrpcModel {
    fn into(self) -> AccountsIntegrationClientAccountGrpcModel {
        AccountsIntegrationClientAccountGrpcModel {
            trader_id: self.trader_id,
            account_id: self.id,
            currency: self.currency,
            balance: self.balance,
            trading_group_id: self.trading_group,
            created_at: self.create_date,
            updated_at: self.last_update_date,
            trading_disabled: self.trading_disabled,
        }
    }
}

/*
impl From<i32> for AccountsIntegrationUpdateAccountBalanceReason {
    fn from(value: i32) -> Self {
        match value {
            0 => AccountsIntegrationUpdateAccountBalanceReason::BalanceCorrection,
            1 => AccountsIntegrationUpdateAccountBalanceReason::Deposit,
            2 => AccountsIntegrationUpdateAccountBalanceReason::Withdrawal,
            _ => panic!("Unknown value for AccountsIntegrationUpdateAccountBalanceReason"),
        }
    }
}
 */

impl Into<UpdateBalanceReason> for AccountsIntegrationUpdateAccountBalanceReason {
    fn into(self) -> UpdateBalanceReason {
        match self {
            AccountsIntegrationUpdateAccountBalanceReason::BalanceCorrection => {
                UpdateBalanceReason::BalanceCorrection
            }
            AccountsIntegrationUpdateAccountBalanceReason::Deposit => UpdateBalanceReason::Deposit,
            AccountsIntegrationUpdateAccountBalanceReason::Withdrawal => {
                UpdateBalanceReason::Withdrawal
            }
        }
    }
}

impl Into<AccountsIntegrationOperationResult> for AccountsManagerOperationResult {
    fn into(self) -> AccountsIntegrationOperationResult {
        match self {
            AccountsManagerOperationResult::Ok => AccountsIntegrationOperationResult::Ok,
            AccountsManagerOperationResult::AccountNotFound => {
                AccountsIntegrationOperationResult::AccountNotFound
            }
            AccountsManagerOperationResult::TraderNotFound => {
                AccountsIntegrationOperationResult::TraderNotFound
            }
            AccountsManagerOperationResult::NotEnoughBalance => {
                AccountsIntegrationOperationResult::NotEnoughBalance
            }
            AccountsManagerOperationResult::ProcessIdDuplicate => {
                AccountsIntegrationOperationResult::ProcessIdDuplicate
            }
        }
    }
}
