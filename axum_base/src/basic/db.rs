use std::sync::atomic::AtomicUsize;
use std::sync::Arc;

use super::env::Env;
use super::Result;

#[derive(Debug)]
struct RoundRobinDatabase<DB: sqlx::Database> {
    database: Vec<sqlx::Pool<DB>>,
    index: AtomicUsize,
}

impl<DB> RoundRobinDatabase<DB>
where
    DB: sqlx::Database,
{
    pub fn new(databse: Vec<sqlx::Pool<DB>>) -> Self {
        RoundRobinDatabase {
            database: databse,
            index: AtomicUsize::new(0),
        }
    }

    pub fn next<'a>(&'a self) -> &'a sqlx::Pool<DB> {
        let len = self.database.len();
        if len == 1 {
            return &self.database[0];
        }
        let mut current = self.index.load(std::sync::atomic::Ordering::Relaxed);
        loop {
            let mut next = current + 1;
            if next >= len {
                next = 0;
            }
            match self.index.compare_exchange_weak(
                current,
                next,
                std::sync::atomic::Ordering::Acquire,
                std::sync::atomic::Ordering::Relaxed,
            ) {
                Ok(_) => return &self.database[current],
                Err(changed) => current = changed,
            }
        }
    }
}

// derive(Clone) not works with generic fields
#[derive(Debug)]
pub struct BasicDatabase<DB: sqlx::Database> {
    sources: Arc<RoundRobinDatabase<DB>>,
    replicas: Arc<RoundRobinDatabase<DB>>,
}

impl<DB> BasicDatabase<DB>
where
    DB: sqlx::Database,
{
    pub fn builder() -> DatabaseBuilder<DB> {
        DatabaseBuilder::<DB> {
            env: Env::default(),
            sources: None,
            replicas: None,
        }
    }

    pub fn read_pool(&self) -> &sqlx::Pool<DB> {
        return self.replicas.next();
    }

    pub fn write_pool(&self) -> &sqlx::Pool<DB> {
        return self.sources.next();
    }
}

pub struct DatabaseBuilder<DB: sqlx::Database> {
    env: Env,
    sources: Option<Vec<sqlx::Pool<DB>>>,
    replicas: Option<Vec<sqlx::Pool<DB>>>,
}

impl<DB> DatabaseBuilder<DB>
where
    DB: sqlx::Database,
{
    pub fn env(mut self, env: Env) -> Self {
        self.env = env;
        self
    }

    pub async fn connect(mut self) -> Result<Self> {
        let mut sources: Vec<sqlx::Pool<_>> = Vec::new();
        for source in self.env.database.sources.iter() {
            sources.push(
                sqlx::pool::PoolOptions::<DB>::new()
                    .max_connections(source.max_connections)
                    .connect(&source.url)
                    .await?,
            )
        }

        let mut replicas: Vec<sqlx::Pool<_>> = Vec::new();
        for replica in self.env.database.replicas.iter() {
            replicas.push(
                sqlx::pool::PoolOptions::<DB>::new()
                    .max_connections(replica.max_connections)
                    .connect(&replica.url)
                    .await?,
            )
        }

        self.sources = Some(sources);
        self.replicas = Some(replicas);

        Ok(self)
    }

    pub fn build(self) -> BasicDatabase<DB> {
        BasicDatabase::<DB> {
            sources: Arc::new(RoundRobinDatabase::new(self.sources.unwrap())),
            replicas: Arc::new(RoundRobinDatabase::new(self.replicas.unwrap())),
        }
    }
}

pub type Database = BasicDatabase<sqlx::Sqlite>;

impl<DB> Clone for BasicDatabase<DB>
where
    DB: sqlx::Database,
{
    fn clone(&self) -> Self {
        Self {
            sources: self.sources.clone(),
            replicas: self.replicas.clone(),
        }
    }
}
