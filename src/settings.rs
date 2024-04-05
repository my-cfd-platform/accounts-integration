use serde_derive::{Deserialize, Serialize};
service_sdk::macros::use_settings!();

use crate::grpc_clients::*;

#[derive(
    service_sdk::my_settings_reader::SettingsModel,
    SdkSettingsTraits,
    AutoGenerateSettingsTraits,
    Serialize,
    Deserialize,
    Debug,
    Clone,
)]
pub struct SettingsModel {
    pub accounts_manager_grpc_url: String,
    pub seq_conn_string: String,
    pub my_telemetry: String,
}

#[async_trait::async_trait]
impl GrpcClientSettings for SettingsReader {
    async fn get_grpc_url(&self, name: &'static str) -> String {
        if name == AccountsManagerGrpcClient::get_service_name() {
            let read_access = self.settings.read().await;
            return read_access.accounts_manager_grpc_url.clone();
        }

        panic!("Unknown grpc service name: {}", name)
    }
}
