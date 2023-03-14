use std::sync::Arc;

use rust_extensions::AppStates;

use crate::{AccountsManagerGrpcClient, SettingsModel};

pub const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const APP_NAME: &'static str = env!("CARGO_PKG_NAME");
pub struct AppContext {
    pub accounts_manager_grpc_client: Arc<AccountsManagerGrpcClient>,
    pub settings: Arc<SettingsModel>,
    pub app_states: Arc<AppStates>,
}

impl AppContext {
    pub async fn new(settings: Arc<SettingsModel>) -> Self {
        let grpc_client =
            AccountsManagerGrpcClient::new(settings.accounts_manager_grpc_service.clone()).await;

        Self {
            accounts_manager_grpc_client: Arc::new(grpc_client),
            settings,
            app_states: Arc::new(AppStates::create_initialized()),
        }
    }
}
