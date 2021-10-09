use std::{env, process::exit};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AkasukiConfig {
    #[serde(alias = "bot")]
    pub discord: DiscordConfig,
    #[serde(default = "default_tracing_config")]
    pub tracing: TracingConfig,
    #[serde(default = "default_postgres_config")]
    #[serde(alias = "database")]
    #[serde(alias = "postgresql")]
    pub postgres: PostgresConfig,
}
#[derive(Debug, Deserialize)]
pub struct DiscordConfig {
    pub token: String,
}

fn default_tracing_level() -> String {
    String::from("info")
}

// Workaround
fn default_as_true() -> bool {
    true
}

fn default_tracing_config() -> TracingConfig {
    TracingConfig {
        enabled: true,
        tracing_level: "info".to_string(),
    }
}

fn default_postgres_config() -> PostgresConfig {
    PostgresConfig {
        host: docker_host(),
        port: default_psql_port(),
        user: docker_user(),
        password: database_password(),
        database: docker_database(),
    }
}

#[derive(Debug, Deserialize)]
pub struct TracingConfig {
    // #[serde(default = "true")]
    #[serde(default = "default_as_true")]
    pub enabled: bool,
    #[serde(default = "default_tracing_level")]
    pub tracing_level: String,
}

fn docker_host() -> String {
    String::from("akasuki-db")
}

fn default_psql_port() -> u16 {
    5432
}

fn docker_user() -> String {
    String::from("postgres")
}

fn database_password() -> String {
    let pwd = env::var("POSTGRES_PASSWORD");
    if let Err(why) = pwd {
        eprintln!("Kuso! Failed to get database password >_<: {}", why);
        exit(-1)
    }

    pwd.unwrap()
}

fn docker_database() -> String {
    String::from("postgres")
}

#[derive(Debug, Deserialize)]
pub struct PostgresConfig {
    #[serde(default = "docker_host")]
    pub host: String,
    #[serde(default = "default_psql_port")]
    pub port: u16,
    #[serde(default = "docker_user")]
    pub user: String,
    #[serde(default = "database_password")]
    pub password: String,
    #[serde(default = "docker_database")]
    pub database: String,
}
