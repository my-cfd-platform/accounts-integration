use service_sdk::my_telemetry::MyTelemetryContext;

use crate::{accounts_manager_grpc::*, AppContext};

pub async fn get_accounts(
    app: &AppContext,
    trader_id: String,
    my_telemetry: &MyTelemetryContext,
) -> Vec<AccountGrpcModel> {
    let accounts = app
        .accounts_manager_grpc_client
        .get_client_accounts(
            AccountManagerGetClientAccountsGrpcRequest { trader_id },
            my_telemetry,
        )
        .await
        .unwrap();

    if let Some(accounts) = accounts {
        accounts
    } else {
        vec![]
    }
}
