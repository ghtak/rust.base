use std::sync::Arc;

use crate::settings::Settings;

#[derive(Debug, Clone)]
pub struct AppContext {
    pub settings: Arc<Settings>,
}

impl AppContext {
    pub fn new(settings: Settings) -> Self {
        AppContext { settings: Arc::new(settings) }
    }
}
