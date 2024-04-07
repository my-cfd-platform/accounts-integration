use service_sdk::my_telemetry::MyTelemetryContext;

use crate::{
    accounts_integration_grpc::{
        AccountsIntegrationAccountBalanceUpdateInfoGrpc, AccountsIntegrationOperationResult,
        AccountsIntegrationUpdateAccountBalanceReason,
    },
    accounts_manager_grpc::UpdateBalanceReason,
    AppContext,
};

use crate::accounts_manager_grpc;

pub async fn update_balance(
    app: &AppContext,
    trader_id: String,
    account_id: String,
    delta: f64,
    allow_negative_balance: bool,
    process_id: String,
    comment: String,
    reason: AccountsIntegrationUpdateAccountBalanceReason,
    reference_transaction_id: Option<String>,
    my_telemetry: &MyTelemetryContext,
) -> Result<AccountsIntegrationAccountBalanceUpdateInfoGrpc, AccountsIntegrationOperationResult> {
    let reason: UpdateBalanceReason = reason.into();

    let result = app
        .accounts_manager_grpc_client
        .update_client_account_balance(
            accounts_manager_grpc::AccountManagerUpdateAccountBalanceGrpcRequest {
                trader_id,
                account_id,
                delta,
                allow_negative_balance,
                process_id,
                comment,
                reason: reason.into(),
                reference_transaction_id,
                same_response_process_id: true, //todo!("What is that?")
            },
            my_telemetry,
        )
        .await
        .unwrap();

    super::mappers::convert_result(result.result(), || {
        result.update_balance_info.unwrap().into()
    })
}

impl Into<AccountsIntegrationAccountBalanceUpdateInfoGrpc>
    for accounts_manager_grpc::AccountManagerUpdateBalanceBalanceGrpcInfo
{
    fn into(self) -> AccountsIntegrationAccountBalanceUpdateInfoGrpc {
        AccountsIntegrationAccountBalanceUpdateInfoGrpc {
            operation_id: self.operation_id,
            account: if let Some(account) = self.account {
                Some(account.into())
            } else {
                None
            },
        }
    }
}
