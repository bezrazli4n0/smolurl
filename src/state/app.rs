use jwt_simple::prelude::*;
use sqlx::postgres::PgPool;

#[derive(Debug, Clone)]
pub struct App {
    pub db_pool: PgPool,
    pub keypair: RS384KeyPair,
}
