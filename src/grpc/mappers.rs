use crate::{
    accounts_integration::{
        AccountsIntegrationClientAccountGrpcModel, AccountsIntegrationOperationResult,
    },
    accounts_manager::{AccountGrpcModel, AccountsManagerOperationResult},
};

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
        }
    }
}
