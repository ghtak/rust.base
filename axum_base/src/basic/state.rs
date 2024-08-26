use crate::oauth2sample::{OAuth2Repository, OAuth2State};

use super::{db::Database, env::Env};


#[derive(Clone, Debug)]
pub struct BasicState {
    pub oauth2_state: OAuth2State,
    pub database: Database,
}

impl BasicState
{
    pub fn new(_env: &Env, database: Database) -> Self {
        BasicState {
            oauth2_state: OAuth2State::new(OAuth2Repository::new()),
            database: database,
        }
    }
}
