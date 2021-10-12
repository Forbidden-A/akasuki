use poise::serenity_prelude::Mutex;
use songbird::SerenityInit;
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
mod utils;

use crate::{
    config::AkasukiConfig,
    database::obtain_postgres_pool,
    global_data::AkasukiData,
    utils::{create_clippy_app, get_application_info},
};

struct LavalinkHandler;

#[lavalink_rs::async_trait]
impl lavalink_rs::gateway::LavalinkEventHandler for LavalinkHandler {
    async fn track_start(
        &self,
        _client: lavalink_rs::LavalinkClient,
        event: lavalink_rs::model::TrackStart,
    ) {
        info!("Track started!\nGuild: {}", event.guild_id);
    }
    async fn track_finish(
        &self,
        _client: lavalink_rs::LavalinkClient,
        event: lavalink_rs::model::TrackFinish,
    ) {
        info!("Track finished!\nGuild: {}", event.guild_id);
    }
}

// Funny little type aliases ;P
type AkasukiError = Box<dyn Error + Send + Sync + 'static>;
type AkasukiContext<'a> = poise::Context<'a, AkasukiData, AkasukiError>;
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
    let lavalink_pwd = if !use_env {
        config.voice.password.to_string()
    } else {
        env::var("LAVALINK_PASSWORD").unwrap_or_else(|_| config.voice.password.to_string())
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
    let app_info = get_application_info(token.as_str()).await?;
    let bot_id = *app_info.id.as_u64();
    let lava_client = lavalink_rs::LavalinkClient::builder(bot_id)
        .set_host(config.voice.host)
        .set_port(config.voice.port)
        .set_password(lavalink_pwd)
        .build(LavalinkHandler)
        .await?;

    let mov_bot_id = bot_id.clone();
    let mut akasuki = poise::Framework::build()
        .prefix("a:")
        .token(&token)
        .user_data_setup(move |_ctx, _ready, _framework| {
            Box::pin(async move {
                Ok(AkasukiData {
                    postgres_pool: Mutex::new(pg_pool),
                    lavalink: lava_client,
                })
            })
        })
        .client_settings(move |builder| {
            builder
                .application_id(mov_bot_id)
                .cache_settings(|cache| cache.max_messages(512))
                .register_songbird()
        });

    akasuki = commands::register(commands::configure(akasuki, &app_info).await?).await?;

    if let Err(why) = akasuki.run().await {
        eprintln!("Oof couldn't start akasuki T-T: {:?}", why);
    }
    Ok(())
}
