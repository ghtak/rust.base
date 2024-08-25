use crate::oauth2sample::{OAuth2Repository, OAuth2State};

use super::env::Env;

#[derive(Clone, Debug)]
pub struct BasicState {
    pub oauth2_state: OAuth2State,
}

impl BasicState {
    pub fn new(_env: &Env) -> Self {
        BasicState {
            oauth2_state: OAuth2State::new(OAuth2Repository::new()),
        }
    }
}
