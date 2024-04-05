use service_sdk::my_telemetry::MyTelemetryContext;

use crate::{
    accounts_integration::{
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
    //todo!("Check Trading Group Validity")
    //todo!("Check Currency Group Validity")
    let response = app
        .accounts_manager_grpc_client
        .create_account(
            accounts_manager_grpc::AccountManagerCreateAccountGrpcRequest {
                trader_id,
                currency,
                trading_group_id,
                process_id,
            },
            my_telemetry,
        )
        .await
        .unwrap();

    Ok(response.into())
}
