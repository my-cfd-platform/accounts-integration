use std::sync::Arc;

use crate::{grpc_clients::AccountsManagerGrpcClient, settings::SettingsReader};

pub const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const APP_NAME: &'static str = env!("CARGO_PKG_NAME");
pub struct AppContext {
    pub accounts_manager_grpc_client: AccountsManagerGrpcClient,
    pub settings_reader: Arc<SettingsReader>,
}

impl AppContext {
    pub async fn new(settings_reader: Arc<SettingsReader>) -> Self {
        Self {
            accounts_manager_grpc_client: AccountsManagerGrpcClient::new(settings_reader.clone()),
            settings_reader,
        }
    }
}
