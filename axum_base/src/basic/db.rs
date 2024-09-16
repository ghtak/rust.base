use std::sync::Arc;

use super::env::{DatabaseConnection, Env};
use super::roundrobin::RoundRobin;
use super::Result;

#[derive(Debug)]
struct BasicDatabaseInner<DB: sqlx::Database> {
    sources: RoundRobin<sqlx::Pool<DB>>,
    replicas: RoundRobin<sqlx::Pool<DB>>,
}

// derive(Clone) not works with generic fields
#[derive(Debug)]
pub struct BasicDatabase<DB: sqlx::Database> {
    inner: Arc<BasicDatabaseInner<DB>>,
}

impl<DB> BasicDatabase<DB>
where
    DB: sqlx::Database,
{
    pub fn builder(env: Env) -> DatabaseBuilder<DB> {
        DatabaseBuilder::<DB> {
            env: env,
            sources: None,
            replicas: None,
        }
    }

    pub fn sources_pool(&self) -> sqlx::Pool<DB> {
        self.inner.sources.next().clone()
    }

    pub fn replicas_pool(&self) -> sqlx::Pool<DB> {
        self.inner.replicas.next().clone()
    }

    pub fn clone_internal(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

// pub trait Database {
//     type Pool;
//     fn sources_pool(&self) -> &Self::Pool;
//     fn replicas_pool(&self) -> &Self::Pool;
// }
// impl<DB> Database for BasicDatabase<DB>
// where
//     DB: sqlx::Database,
// {
//     type Pool = sqlx::Pool<DB>;

//     fn sources_pool(&self) -> &sqlx::Pool<DB> {
//         return self.sources.next();
//     }

//     fn replicas_pool(&self) -> &sqlx::Pool<DB> {
//         return self.replicas.next();
//     }
// }

pub struct DatabaseBuilder<DB: sqlx::Database> {
    env: Env,
    sources: Option<Vec<sqlx::Pool<DB>>>,
    replicas: Option<Vec<sqlx::Pool<DB>>>,
}

impl<DB> DatabaseBuilder<DB>
where
    DB: sqlx::Database,
{
    async fn _connect_db(
        &self,
        conn_infos: &Vec<DatabaseConnection>,
    ) -> Result<Vec<sqlx::Pool<DB>>> {
        let mut pools: Vec<sqlx::Pool<DB>> = Vec::new();
        for conn_info in conn_infos.iter() {
            pools.push(
                sqlx::pool::PoolOptions::<DB>::new()
                    .max_connections(conn_info.max_connections)
                    .connect(&conn_info.url)
                    .await?,
            )
        }
        Ok(pools)
    }

    pub async fn connect(mut self) -> Result<Self> {
        self.sources = Some(self._connect_db(&self.env.database.sources).await?);
        self.replicas = Some(self._connect_db(&self.env.database.replicas).await?);
        Ok(self)
    }

    pub fn build(self) -> BasicDatabase<DB> {
        BasicDatabase {
            inner: Arc::new(BasicDatabaseInner {
                sources: RoundRobin::new(self.sources.unwrap()),
                replicas: RoundRobin::new(self.replicas.unwrap()),
            }),
        }
    }
}
