use crate::app::AppContext;
use std::sync::Arc;

#[derive(Clone)]
pub struct SdkGrpcService {
    pub app: Arc<AppContext>,
}

impl SdkGrpcService {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}
