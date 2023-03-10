use std::sync::Arc;

use accounts_integration::{start_grpc_server, AppContext, SettingsReader};

#[tokio::main]
async fn main() {
    let settings_reader = SettingsReader::new(".yourfin").await;
    let settings_model = Arc::new(settings_reader.get_settings().await);
    let app = Arc::new(AppContext::new(settings_model).await);

    start_grpc_server(app.clone(), 8888).await;

    app.app_states.wait_until_shutdown().await;
}
 