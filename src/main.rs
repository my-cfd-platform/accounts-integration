mod app;
mod flows;
mod grpc_clients;
mod grpc_server;
mod settings;

pub mod accounts_manager_grpc {
    tonic::include_proto!("accounts_manager");
}

pub mod accounts_integration {
    tonic::include_proto!("accounts_integration");
}

pub use app::*;
pub use grpc_server::*;

#[tokio::main]
async fn main() {
    let settings_reader = crate::settings::SettingsReader::new(".my-cfd-platform").await;
}
