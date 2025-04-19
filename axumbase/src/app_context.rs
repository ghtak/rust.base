use crate::settings::Settings;

#[derive(Debug, Clone)]
pub struct AppContext {
    pub settings: Settings,
}

impl AppContext {
    pub fn new(settings: Settings) -> Self {
        AppContext { settings }
    }
}
