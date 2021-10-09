use sqlx::postgres::{PgConnectOptions, PgPool, PgPoolOptions};
use std::env;

use crate::AkasukiResult;

// This function obtains a database pool for the postgresql database used for the bot.
pub async fn obtain_postgres_pool() -> AkasukiResult<PgPool> {
    // Obtain the postgresql url.
    let pg_uri = env::var("POSTGRES_URI");

    // Connect to the database with the information provided on the configuration.
    // and return a pool of connections
    let pool_options = PgPoolOptions::new().max_connections(20);
    let pool = if let Ok(uri) = pg_uri {
        pool_options.connect(uri.as_str()).await
    } else {
        let connect_options = PgConnectOptions::new().host("");
        pool_options.connect_with(connect_options).await
    }?;

    // return the pool
    Ok(pool)
}
