mod app;
mod grpc;

pub mod accounts_manager {
    tonic::include_proto!("accounts_manager");
}

pub mod accounts_integration {
    tonic::include_proto!("accounts_integration");
}

pub use app::*;
pub use grpc::*;
