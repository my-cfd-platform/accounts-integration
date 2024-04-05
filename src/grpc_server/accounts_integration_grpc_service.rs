service_sdk::macros::use_grpc_server!();

use crate::accounts_integration::accounts_integration_grpc_service_server::AccountsIntegrationGrpcService;
use crate::accounts_integration::*;
use crate::accounts_manager_grpc;

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

        todo!("Implement")
        /*
        let result = self
            .app
            .accounts_manager_grpc_client
            .update_client_account_balance(
                accounts_manager_grpc::AccountManagerUpdateAccountBalanceGrpcRequest {
                    trader_id: request.trader_id,
                    account_id: request.account_id,
                    delta: request.delta,
                    allow_negative_balance: request.allow_negative_balance,
                    process_id: request.process_id,
                    comment: request.comment,
                    reason: request.reason(),
                },
                my_telemetry,
            )
            .await
            .unwrap();

        let response =
            super::mappers::convert_result(result.result(), || result.update_balance_info.unwrap());

        let response = match response {
            Ok(data) => AccountsIntegrationAccountUpdateBalanceGrpcResponse {
                update_balance_info: Some(AccountsIntegrationAccountBalanceUpdateInfoGrpc {
                    operation_id: data.operation_id,
                    account: Some(data.account.unwrap().into()),
                }),
                result: 0,
            },
            Err(error) => AccountsIntegrationAccountUpdateBalanceGrpcResponse {
                update_balance_info: None,
                result: error.into(),
            },
        };

        Ok(tonic::Response::new(response))
         */
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
