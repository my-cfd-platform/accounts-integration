use service_sdk::my_telemetry::MyTelemetryContext;

use crate::{
    accounts_integration_grpc::AccountsIntegrationOperationResult,
    accounts_manager_grpc::{AccountGrpcModel, AccountManagerUpdateTradingDisabledGrpcRequest},
    AppContext,
};

pub async fn update_account_trading_disabled(
    app: &AppContext,
    trader_id: String,
    account_id: String,
    process_id: String,
    trading_disabled: bool,
    my_telemetry: &MyTelemetryContext,
) -> Result<AccountGrpcModel, AccountsIntegrationOperationResult> {
    let response = app
        .accounts_manager_grpc_client
        .update_account_trading_disabled(
            AccountManagerUpdateTradingDisabledGrpcRequest {
                trader_id,
                account_id,
                trading_disabled: trading_disabled,
                process_id,
            },
            my_telemetry,
        )
        .await
        .unwrap();

    super::mappers::convert_result(response.result(), || response.account.unwrap())
}
