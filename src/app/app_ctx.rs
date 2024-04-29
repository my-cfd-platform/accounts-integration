use std::sync::Arc;

use my_nosql_contracts::{IntegrationSettingsMyNoSqlEntity, TradingGroupNoSqlEntity};
use service_sdk::{my_no_sql_sdk::reader::MyNoSqlDataReaderTcp, ServiceContext};

use crate::{grpc_clients::AccountsManagerGrpcClient, settings::SettingsReader};

pub const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const APP_NAME: &'static str = env!("CARGO_PKG_NAME");
pub struct AppContext {
    pub accounts_manager_grpc_client: AccountsManagerGrpcClient,
    pub trading_groups: Arc<MyNoSqlDataReaderTcp<TradingGroupNoSqlEntity>>,
    pub settings_reader: Arc<SettingsReader>,
    pub integration_settings: Arc<MyNoSqlDataReaderTcp<IntegrationSettingsMyNoSqlEntity>>,
}

impl AppContext {
    pub async fn new(
        settings_reader: Arc<SettingsReader>,
        service_context: &ServiceContext,
    ) -> Self {
        Self {
            accounts_manager_grpc_client: AccountsManagerGrpcClient::new(settings_reader.clone()),
            settings_reader,
            trading_groups: service_context.get_ns_reader().await,
            integration_settings: service_context.get_ns_reader().await,
        }
    }

    pub async fn check_api_key(&self, api_key: &str) {
        let entity = self
            .integration_settings
            .get_entity(
                IntegrationSettingsMyNoSqlEntity::generate_partition_key(),
                IntegrationSettingsMyNoSqlEntity::generate_row_key(api_key),
            )
            .await;

        if entity.is_none() {
            panic!("Invalid API key");
        }
    }
}
