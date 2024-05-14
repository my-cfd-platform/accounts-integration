use service_sdk::my_telemetry::MyTelemetryContext;

use crate::{
    accounts_integration_grpc::{
        AccountsIntegrationClientAccountGrpcModel, AccountsIntegrationOperationResult,
    },
    AppContext,
};

use crate::accounts_manager_grpc;

pub async fn create_account(
    app: &AppContext,
    trader_id: String,
    currency: String,
    trading_group_id: Option<String>,
    process_id: String,
    my_telemetry: &MyTelemetryContext,
) -> Result<AccountsIntegrationClientAccountGrpcModel, AccountsIntegrationOperationResult> {
    if let Some(trading_group_id) = trading_group_id.as_ref() {
        crate::internal_scripts::check_trading_group(app, trading_group_id).await?;
    }

    crate::internal_scripts::check_account_currency(currency.as_str())?;

    let response = app
        .accounts_manager_grpc_client
        .create_account(
            accounts_manager_grpc::AccountManagerCreateAccountGrpcRequest {
                trader_id,
                currency,
                trading_group_id,
                process_id,
                metadata: vec![],
            },
            my_telemetry,
        )
        .await
        .unwrap();

    Ok(response.into())
}
