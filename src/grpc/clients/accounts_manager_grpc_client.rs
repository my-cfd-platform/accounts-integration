use my_grpc_extensions::GrpcClientInterceptor;
use my_telemetry::MyTelemetryContext;
use std::time::Duration;
use tonic::{codegen::InterceptedService, transport::Channel};

use crate::accounts_manager::{
    accounts_manager_grpc_service_client::AccountsManagerGrpcServiceClient, AccountGrpcModel,
    AccountManagerCreateAccountGrpcRequest, AccountManagerGetClientAccountGrpcRequest,
    AccountManagerGetClientAccountsGrpcRequest, AccountManagerUpdateAccountBalanceGrpcRequest,
    AccountManagerUpdateTradingDisabledGrpcRequest, AccountsManagerOperationResult,
};

pub struct AccountsManagerGrpcClient {
    channel: Channel,
    timeout: Duration,
}

impl AccountsManagerGrpcClient {
    pub async fn new(grpc_address: String) -> Self {
        let channel = Channel::from_shared(grpc_address)
            .unwrap()
            .connect()
            .await
            .unwrap();
        Self {
            channel,
            timeout: Duration::from_secs(5),
        }
    }

    fn create_grpc_service(
        &self,
        my_telemetry_context: &MyTelemetryContext,
    ) -> AccountsManagerGrpcServiceClient<InterceptedService<Channel, GrpcClientInterceptor>> {
        return AccountsManagerGrpcServiceClient::with_interceptor(
            self.channel.clone(),
            GrpcClientInterceptor::new(my_telemetry_context.clone()),
        );
    }

    pub async fn create_client_account(
        &self,
        trader_id: String,
        process_id: String,
        currency: String,
        trading_group_id: Option<String>,
        my_telemetry_context: &MyTelemetryContext,
    ) -> AccountGrpcModel {
        let mut client = self.create_grpc_service(my_telemetry_context);
        let future = client.create_account(AccountManagerCreateAccountGrpcRequest {
            trader_id,
            currency,
            process_id,
            trading_group_id,
        });

        let response = tokio::time::timeout(self.timeout, future)
            .await
            .unwrap()
            .unwrap();

        return response.into_inner();
    }

    pub async fn update_client_account_balance(
        &self,
        trader_id: String,
        account_id: String,
        delta: f64,
        allow_negative_balance: bool,
        process_id: String,
        comment: String,
        my_telemetry_context: &MyTelemetryContext,
    ) -> Result<AccountGrpcModel, AccountsManagerOperationResult> {
        let mut client = self.create_grpc_service(my_telemetry_context);
        let future =
            client.update_client_account_balance(AccountManagerUpdateAccountBalanceGrpcRequest {
                trader_id,
                account_id,
                delta,
                comment,
                process_id,
                allow_negative_balance,
            });

        let response = tokio::time::timeout(self.timeout, future)
            .await
            .unwrap()
            .unwrap();

        let response = response.into_inner();

        let to_return = match response.result {
            0 => Ok(response.account.unwrap()),
            1 => Err(AccountsManagerOperationResult::AccountNotFound),
            2 => Err(AccountsManagerOperationResult::TraderNotFound),
            3 => Err(AccountsManagerOperationResult::NotEnoughBalance),
            _ => panic!("Unknown status from accounts manager"),
        };

        return to_return;
    }

    pub async fn update_client_trading_disabled(
        &self,
        trader_id: String,
        account_id: String,
        trading_disabled: bool,
        process_id: String,
        my_telemetry_context: &MyTelemetryContext,
    ) -> Result<AccountGrpcModel, AccountsManagerOperationResult> {
        let mut client = self.create_grpc_service(my_telemetry_context);
        let future = client.update_account_trading_disabled(
            AccountManagerUpdateTradingDisabledGrpcRequest {
                trader_id,
                account_id,
                trading_disabled,
                process_id,
            },
        );

        let response = tokio::time::timeout(self.timeout, future)
            .await
            .unwrap()
            .unwrap();

        let response = response.into_inner();

        let to_return = match response.result {
            0 => Ok(response.account.unwrap()),
            1 => Err(AccountsManagerOperationResult::AccountNotFound),
            2 => Err(AccountsManagerOperationResult::TraderNotFound),
            3 => Err(AccountsManagerOperationResult::NotEnoughBalance),
            _ => panic!("Unknown status from accounts manager"),
        };

        return to_return;
    }

    pub async fn get_client_account(
        &self,
        trader_id: String,
        account_id: String,
        my_telemetry_context: &MyTelemetryContext,
    ) -> Result<AccountGrpcModel, AccountsManagerOperationResult> {
        let mut client = self.create_grpc_service(my_telemetry_context);
        let future = client.get_client_account(AccountManagerGetClientAccountGrpcRequest {
            trader_id,
            account_id,
        });

        let response = tokio::time::timeout(self.timeout, future)
            .await
            .unwrap()
            .unwrap();

        let response = response.into_inner();

        let to_return = match response.result {
            0 => Ok(response.account.unwrap()),
            1 => Err(AccountsManagerOperationResult::AccountNotFound),
            2 => Err(AccountsManagerOperationResult::TraderNotFound),
            3 => Err(AccountsManagerOperationResult::NotEnoughBalance),
            _ => panic!("Unknown status from accounts manager"),
        };

        return to_return;
    }

    pub async fn get_client_accounts(
        &self,
        trader_id: String,
        my_telemetry_context: &MyTelemetryContext,
    ) -> Vec<AccountGrpcModel> {
        let mut client = self.create_grpc_service(my_telemetry_context);
        let future =
            client.get_client_accounts(AccountManagerGetClientAccountsGrpcRequest { trader_id });

        let response = tokio::time::timeout(self.timeout, future)
            .await
            .unwrap()
            .unwrap();

        let response = response.into_inner();
        return my_grpc_extensions::read_grpc_stream::as_vec(response, self.timeout)
            .await
            .unwrap()
            .unwrap();
    }
}
