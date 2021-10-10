use clap::{
    crate_authors, crate_description, crate_name, crate_version, App as ClippyApp, AppSettings, Arg,
};
use poise::serenity_prelude::Mutex;
use std::{env, error::Error};
use tokio::{fs::File, io::AsyncReadExt};
use tracing::{info, instrument, Level};
use tracing_log::LogTracer;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

mod commands;
mod config;
mod database;
mod global_data;
mod listeners;

use crate::{config::AkasukiConfig, database::obtain_postgres_pool, global_data::AkasukiData};

fn create_clippy_app(no_color: Option<bool>) -> ClippyApp<'static> {
    let clap_color_setting = if env::var_os("NO_COLOR").is_none() && !no_color.unwrap_or(false) {
        AppSettings::ColoredHelp
    } else {
        AppSettings::ColorNever
    };

    let app = ClippyApp::new(crate_name!())
        .version(crate_version!())
        .global_setting(clap_color_setting)
        .about(crate_description!())
        .author(crate_authors!())
        .arg(
            Arg::new("config")
                .short('c')
                .alias("cfg")
                .value_name("FILE")
                .about("Location of Config.toml")
                .default_value("./Config.toml"),
        )
        .arg(
            Arg::new("use_env")
                .takes_value(false)
                .short('e')
                .about("Whether or not the bot should use environment variable, this flag will be ignored if '.env' is present."),
        );

    app
}

// Funny little type aliases ;P
type AkasukiError = Box<dyn Error + Send + Sync + 'static>;
type Context<'a> = poise::Context<'a, AkasukiData, AkasukiError>;
type AkasukiResult<R> = Result<R, AkasukiError>;

#[tokio::main]
#[instrument]
async fn main() -> AkasukiResult<()> {
    let options = create_clippy_app(Some(env::var_os("NO_COLOR").is_some())).get_matches();
    let use_env = dotenv::dotenv().is_ok() || options.is_present("use_env");
    // using tokio file for fun haha
    let mut config_file = File::open(options.value_of("config").unwrap()).await?;
    let mut config_content = String::new();
    config_file.read_to_string(&mut config_content).await?;
    let config = toml::from_str::<AkasukiConfig>(&config_content).unwrap();
    let token = if !use_env {
        config.discord.token.to_string()
    } else {
        env::var("AKASUKI_TOKEN").unwrap_or_else(|_| config.discord.token.to_string())
    };

    if config.tracing.enabled {
        LogTracer::init()?;
        let level = match config.tracing.tracing_level.as_str() {
            "error" => Level::ERROR,
            "warn" => Level::WARN,
            "info" => Level::INFO,
            "debug" => Level::DEBUG,
            "trace" => Level::TRACE,
            _ => Level::INFO,
        };

        info!("Tracer initialized with level {}.", level);

        if use_env {
            let subscriber = FmtSubscriber::builder()
                .with_env_filter(EnvFilter::from_default_env())
                .with_max_level(level)
                .finish();
            tracing::subscriber::set_global_default(subscriber)?;
        } else {
            let subscriber = FmtSubscriber::builder().with_max_level(level).finish();
            tracing::subscriber::set_global_default(subscriber)?;
        };

        info!("Subscriber initialized.");
    }

    let pg_pool = obtain_postgres_pool(config.postgres).await?;
    sqlx::migrate!().run(&pg_pool).await?;

    let mut akasuki = poise::Framework::build()
        .prefix("a:")
        .token(&token)
        .user_data_setup(move |_ctx, _ready, _framework| {
            Box::pin(async move {
                Ok(AkasukiData {
                    postgres_pool: Mutex::new(pg_pool),
                })
            })
        });

    akasuki = commands::register(commands::configure(akasuki, &token).await?).await?;

    if let Err(why) = akasuki.run().await {
        eprintln!("Oof couldn't start akasuki T-T: {:?}", why);
    }
    Ok(())
}
