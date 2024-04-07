mod app;
mod flows;
mod grpc_clients;
mod grpc_server;
mod internal_scripts;
mod settings;

use accounts_integration_grpc::accounts_integration_grpc_service_server::AccountsIntegrationGrpcServiceServer;

pub mod accounts_manager_grpc {
    tonic::include_proto!("accounts_manager");
}

pub mod accounts_integration_grpc {
    tonic::include_proto!("accounts_integration");
}

use std::sync::Arc;

pub use app::*;
pub use grpc_server::*;

#[tokio::main]
async fn main() {
    let settings_reader = crate::settings::SettingsReader::new(".my-cfd-platform").await;

    let settings_reader = Arc::new(settings_reader);

    let mut service_context = service_sdk::ServiceContext::new(settings_reader.clone()).await;

    let app = AppContext::new(settings_reader, &service_context).await;
    let app = Arc::new(app);

    let grpc_service = SdkGrpcService::new(app.clone());

    service_context.configure_grpc_server(|builder| {
        builder.add_grpc_service(AccountsIntegrationGrpcServiceServer::new(
            grpc_service.clone(),
        ))
    });

    service_context.start_application().await;
}
