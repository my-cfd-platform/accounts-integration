use std::pin::Pin;

use rust_extensions::date_time::DateTimeAsMicroseconds;

use crate::accounts_integration::{
    accounts_integration_grpc_service_server::AccountsIntegrationGrpcService,
    AccountsIntegrationAccountGrpcResponse, AccountsIntegrationClientAccountGrpcModel,
    AccountsIntegrationCreateClientAccountGrpcRequest,
    AccountsIntegrationGetClientAccountsGrpcRequest,
    AccountsIntegrationUpdateAccountBalanceGrpcRequest,
    AccountsIntegrationUpdateAccountTradingDisabledGrpcRequest, PingResponse,
};

use super::server::GrpcService;

#[tonic::async_trait]
impl AccountsIntegrationGrpcService for GrpcService {
    type GetClientAccountsStream = Pin<
        Box<
            dyn tonic::codegen::futures_core::Stream<
                    Item = Result<AccountsIntegrationClientAccountGrpcModel, tonic::Status>,
                > + Send
                + Sync
                + 'static,
        >,
    >;

    async fn create_client_account(
        &self,
        request: tonic::Request<AccountsIntegrationCreateClientAccountGrpcRequest>,
    ) -> Result<tonic::Response<AccountsIntegrationAccountGrpcResponse>, tonic::Status> {
        let my_telemetry_context = my_grpc_extensions::get_telemetry(
            &request.metadata(),
            request.remote_addr(),
            "create_client_account",
        );

        let AccountsIntegrationCreateClientAccountGrpcRequest {
            trader_id,
            currency,
            process_id,
        } = request.into_inner();

        let account = self
            .app
            .accounts_manager_grpc_client
            .create_client_account(
                trader_id,
                process_id,
                currency,
                my_telemetry_context.get_ctx(),
            )
            .await;

        let response = AccountsIntegrationAccountGrpcResponse {
            result: 0,
            account: Some(account.into()),
        };

        Ok(tonic::Response::new(response))
    }

    async fn update_client_account_balance(
        &self,
        request: tonic::Request<AccountsIntegrationUpdateAccountBalanceGrpcRequest>,
    ) -> Result<tonic::Response<AccountsIntegrationAccountGrpcResponse>, tonic::Status> {
        let my_telemetry_context = my_grpc_extensions::get_telemetry(
            &request.metadata(),
            request.remote_addr(),
            "create_client_account",
        );

        let AccountsIntegrationUpdateAccountBalanceGrpcRequest {
            trader_id,
            account_id,
            delta,
            comment,
            process_id,
            allow_negative_balance,
        } = request.into_inner();

        let account = self
            .app
            .accounts_manager_grpc_client
            .update_client_account_balance(
                trader_id,
                account_id,
                delta,
                allow_negative_balance,
                process_id,
                comment,
                my_telemetry_context.get_ctx(),
            )
            .await;

        let response = match account {
            Ok(account) => AccountsIntegrationAccountGrpcResponse {
                account: Some(account.into()),
                result: 0,
            },
            Err(error) => AccountsIntegrationAccountGrpcResponse {
                account: None,
                result: error.into(),
            },
        };

        Ok(tonic::Response::new(response))
    }

    async fn update_account_trading_disabled(
        &self,
        request: tonic::Request<AccountsIntegrationUpdateAccountTradingDisabledGrpcRequest>,
    ) -> Result<tonic::Response<AccountsIntegrationAccountGrpcResponse>, tonic::Status> {
        let my_telemetry_context = my_grpc_extensions::get_telemetry(
            &request.metadata(),
            request.remote_addr(),
            "create_client_account",
        );

        let AccountsIntegrationUpdateAccountTradingDisabledGrpcRequest {
            trader_id,
            account_id,
            trading_disabled,
            process_id,
        } = request.into_inner();

        let account = self
            .app
            .accounts_manager_grpc_client
            .update_client_trading_disabled(
                trader_id,
                account_id,
                trading_disabled,
                process_id,
                my_telemetry_context.get_ctx(),
            )
            .await;

        let response = match account {
            Ok(account) => AccountsIntegrationAccountGrpcResponse {
                account: Some(account.into()),
                result: 0,
            },
            Err(error) => AccountsIntegrationAccountGrpcResponse {
                account: None,
                result: error.into(),
            },
        };

        Ok(tonic::Response::new(response))
    }

    async fn get_client_accounts(
        &self,
        request: tonic::Request<AccountsIntegrationGetClientAccountsGrpcRequest>,
    ) -> Result<tonic::Response<Self::GetClientAccountsStream>, tonic::Status> {
        let my_telemetry_context = my_grpc_extensions::get_telemetry(
            &request.metadata(),
            request.remote_addr(),
            "create_client_account",
        );

        let AccountsIntegrationGetClientAccountsGrpcRequest { trader_id } = request.into_inner();

        let accounts = self
            .app
            .accounts_manager_grpc_client
            .get_client_accounts(trader_id, my_telemetry_context.get_ctx())
            .await;

        my_grpc_extensions::grpc_server::send_vec_to_stream(accounts, |x| x.into()).await
    }

    async fn ping(
        &self,
        _: tonic::Request<()>,
    ) -> Result<tonic::Response<PingResponse>, tonic::Status> {
       
        return Ok(tonic::Response::new(PingResponse{
            service_name: "ACCOUNTS_INTEGRATION".to_string(),
            date_time: DateTimeAsMicroseconds::now().unix_microseconds as u64,
        }));
    }
}
