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
        println!("Requested trading group: {}", trading_group_id);

        let groups = app
            .trading_groups
            .get_by_partition_key(TradingGroupNoSqlEntity::generate_partition_key())
            .await;

        match groups {
            Some(entities) => {
                for entity in entities.values() {
                    println!(
                        "Entity in dictionary: PK:{}, RK:{}",
                        entity.partition_key, entity.row_key
                    );
                }
            }
            None => {
                println!("No trading groups in dictionary");
            }
        }

        return Err(AccountsIntegrationOperationResult::InvalidTradingGroup);
    }

    Ok(())
}
