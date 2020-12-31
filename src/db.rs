use crate::error::Error;
use crate::error::Error::DBPoolError;
use mobc::{Connection, Pool};
use mobc_postgres::{tokio_postgres, PgConnectionManager};
use std::convert::Infallible;
use std::str::FromStr;
use std::time::Duration;
use tokio_postgres::{Config, NoTls};
use warp::Filter;

const DB_POOL_MAX_OPEN: u64 = 32;
const DB_POOL_MAX_IDLE: u64 = 8;
const DB_POOL_TIMEOUT_SECONDS: u64 = 15;

pub type DBCon = Connection<PgConnectionManager<NoTls>>;
pub type DBPool = Pool<PgConnectionManager<NoTls>>;

pub fn with_db(
    db_pool: DBPool,
) -> impl Filter<Extract = (DBPool,), Error = Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}

pub async fn init() -> DBPool {
    let db_pool = create_pool().expect("database pool can be created");
    init_db(&db_pool)
        .await
        .expect("database can be initialized");
    db_pool
}

pub fn create_pool() -> std::result::Result<DBPool, Error> {
    let config = Config::from_str("postgres://postgres@127.0.0.1:7878/postgres")?;

    let manager = PgConnectionManager::new(config, NoTls);
    Ok(DBPool::builder()
        .max_open(DB_POOL_MAX_OPEN)
        .max_idle(DB_POOL_MAX_IDLE)
        .get_timeout(Some(Duration::from_secs(DB_POOL_TIMEOUT_SECONDS)))
        .build(manager))
}

const INIT_SQL: &str = include_str!("init.sql");

pub async fn get_db_conn(db_pool: &DBPool) -> Result<DBCon, Error> {
    db_pool.get().await.map_err(DBPoolError)
}

pub async fn init_db(db_pool: &DBPool) -> Result<(), Error> {
    let con = get_db_conn(db_pool).await?;
    con.batch_execute(INIT_SQL).await?;
    Ok(())
}
