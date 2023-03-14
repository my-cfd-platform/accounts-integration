use my_seq_logger::SeqSettings;
use my_settings_reader::SettingsModel;
use serde_derive::{Deserialize, Serialize};

#[derive(SettingsModel, Serialize, Deserialize, Debug, Clone)]
pub struct SettingsModel {
    #[serde(rename = "AccountsManagerGrpcService")]
    pub accounts_manager_grpc_service: String,
    #[serde(rename = "Seq")]
    pub seq: String,
}

#[async_trait::async_trait]
impl SeqSettings for SettingsModel {
    async fn get_conn_string(&self) -> String {
        self.seq.clone()
    }
}
