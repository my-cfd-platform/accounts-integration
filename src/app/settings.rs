use my_settings_reader::SettingsModel;
use serde_derive::{Serialize, Deserialize};

#[derive(SettingsModel, Serialize, Deserialize, Debug, Clone)]
pub struct SettingsModel{
    #[serde(rename = "AccountsManagerGrpcService")]
    pub accounts_manager_grpc_service: String,
}