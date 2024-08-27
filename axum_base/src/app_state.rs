use crate::basic::db::BasicDatabase;

pub type DatabaseDriver = sqlx::Sqlite;
pub type Database = BasicDatabase<DatabaseDriver>;

#[derive(Clone, Debug)]
pub struct AppState {
    pub database: Database,
}

impl AppState {
    pub fn new(database: Database) -> Self {
        AppState { database }
    }
}

impl Clone for Database {
    fn clone(&self) -> Self {
        self.clone_internal()
    }
}
