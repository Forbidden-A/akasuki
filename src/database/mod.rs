pub mod models;

use sqlx::postgres::{PgConnectOptions, PgPool, PgPoolOptions};

use crate::{config::PostgresConfig, AkasukiResult};

pub async fn obtain_postgres_pool(config: PostgresConfig) -> AkasukiResult<PgPool> {
    // Connect to the database with the information provided in the config.
    // and return a pool of connections
    let connect_options = PgConnectOptions::new()
        .host(&config.host)
        .port(config.port)
        .username(&config.user)
        .password(&config.password)
        .database(&config.database);

    println!("{:?}", connect_options);
    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect_with(connect_options)
        .await?;

    // return the pool
    Ok(pool)
}
