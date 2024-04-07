use crate::accounts_integration_grpc::AccountsIntegrationOperationResult;

pub fn check_account_currency(
    account_currency: &str,
) -> Result<(), AccountsIntegrationOperationResult> {
    if account_currency.to_uppercase() != "USD" {
        //todo!("Move to dictionary")
        return Err(AccountsIntegrationOperationResult::InvalidAccountCurrency);
    }

    Ok(())
}
