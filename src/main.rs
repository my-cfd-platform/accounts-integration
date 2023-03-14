use std::sync::Arc;

use accounts_integration::{start_grpc_server, AppContext, SettingsReader, APP_NAME, APP_VERSION};
use my_seq_logger::SeqLogger;

#[tokio::main]
async fn main() {
    let settings_reader = SettingsReader::new(".yourfin").await;
    let settings_model = Arc::new(settings_reader.get_settings().await);
    let app = Arc::new(AppContext::new(settings_model.clone()).await);

    SeqLogger::enable_from_connection_string(
        APP_NAME.to_string(),
        APP_VERSION.to_string(),
        settings_model.clone(),
        None,
    );

    start_grpc_server(app.clone(), 8888).await;

    app.app_states.wait_until_shutdown().await;
}
