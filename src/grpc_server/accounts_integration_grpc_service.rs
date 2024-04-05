service_sdk::macros::use_grpc_server!();

use crate::accounts_integration::accounts_integration_grpc_service_server::AccountsIntegrationGrpcService;
use crate::accounts_integration::*;

use super::server::GrpcService;

#[tonic::async_trait]
impl AccountsIntegrationGrpcService for GrpcService {
    #[with_telemetry]
    async fn create_client_account(
        &self,
        request: tonic::Request<AccountsIntegrationCreateClientAccountGrpcRequest>,
    ) -> Result<tonic::Response<AccountsIntegrationAccountGrpcResponse>, tonic::Status> {
        let request = request.into_inner();

        let response = crate::flows::create_account(
            &self.app,
            request.trader_id,
            request.currency,
            request.trading_group_id,
            request.process_id,
            my_telemetry,
        )
        .await;

        let response = match response {
            Ok(account) => AccountsIntegrationAccountGrpcResponse {
                result: 0,
                account: Some(account),
            },
            Err(error) => AccountsIntegrationAccountGrpcResponse {
                result: error.into(),
                account: None,
            },
        };

        Ok(tonic::Response::new(response))
    }

    #[with_telemetry]
    async fn update_client_account_balance(
        &self,
        request: tonic::Request<AccountsIntegrationUpdateAccountBalanceGrpcRequest>,
    ) -> Result<tonic::Response<AccountsIntegrationAccountUpdateBalanceGrpcResponse>, tonic::Status>
    {
        let request = request.into_inner();

        let reason = request.reason();

        let response = crate::flows::update_balance(
            &self.app,
            request.trader_id,
            request.account_id,
            request.delta,
            request.allow_negative_balance,
            request.process_id,
            request.comment,
            reason,
            request.reference_transaction_id,
            my_telemetry,
        )
        .await;

        let response = match response {
            Ok(model) => AccountsIntegrationAccountUpdateBalanceGrpcResponse {
                result: 0,
                update_balance_info: Some(model),
            },
            Err(error) => AccountsIntegrationAccountUpdateBalanceGrpcResponse {
                result: error.into(),
                update_balance_info: None,
            },
        };

        Ok(tonic::Response::new(response))
    }

    #[with_telemetry]
    async fn update_account_trading_disabled(
        &self,
        request: tonic::Request<AccountsIntegrationUpdateAccountTradingDisabledGrpcRequest>,
    ) -> Result<tonic::Response<AccountsIntegrationAccountGrpcResponse>, tonic::Status> {
        let request = request.into_inner();
        todo!("Implement")

        /*
        let response = self
            .app
            .accounts_manager_grpc_client
            .update_account_trading_disabled(
                AccountsIntegrationUpdateAccountTradingDisabledGrpcRequest {
                    trader_id: request.trader_id,
                    account_id: request.account_id,
                    trading_disabled: request.trading_disabled,
                    process_id: request.process_id,
                },
                my_telemetry,
            )
            .await
            .unwrap();

        let response =
            super::mappers::convert_result(response.result(), || response.account.unwrap());

        let response = match response {
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
         */
    }

    generate_server_stream!(stream_name:"GetClientAccountsStream", item_name:"AccountsIntegrationClientAccountGrpcModel");

    #[with_telemetry]
    async fn get_client_accounts(
        &self,
        request: tonic::Request<AccountsIntegrationGetClientAccountsGrpcRequest>,
    ) -> Result<tonic::Response<Self::GetClientAccountsStream>, tonic::Status> {
        let request = request.into_inner();

        todo!("Implement")
        /*
        let accounts = self
            .app
            .accounts_manager_grpc_client
            .get_client_accounts(
                AccountsIntegrationGetClientAccountsGrpcRequest {
                    trader_id: request.trader_id,
                },
                my_telemetry,
            )
            .await
            .unwrap();

        let accounts = if let Some(accounts) = accounts {
            accounts
        } else {
            vec![]
        };

        my_grpc_extensions::grpc_server::send_vec_to_stream(accounts.into_iter(), |x| x.into())
            .await
             */
    }

    async fn ping(&self, _: tonic::Request<()>) -> Result<tonic::Response<()>, tonic::Status> {
        Ok(tonic::Response::new(()))
    }
}