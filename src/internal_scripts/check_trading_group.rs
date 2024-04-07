use my_nosql_contracts::TradingGroupNoSqlEntity;

use crate::{accounts_integration_grpc::AccountsIntegrationOperationResult, AppContext};

pub async fn check_trading_group(
    app: &AppContext,
    trading_group_id: &str,
) -> Result<(), AccountsIntegrationOperationResult> {
    if app
        .trading_groups
        .get_entity(
            TradingGroupNoSqlEntity::generate_partition_key(),
            trading_group_id,
        )
        .await
        .is_some()
    {
        return Err(AccountsIntegrationOperationResult::InvalidTradingGroup);
    }

    Ok(())
}
