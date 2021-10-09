use clap::{
    crate_authors, crate_description, crate_name, crate_version, App as ClippyApp, AppSettings, Arg,
};
use serenity::Client;

use std::{env, error::Error};
use tokio::{fs::File, io::AsyncReadExt};
use tracing::instrument;

mod commands;
mod config;
mod events;

use crate::config::AkasukiConfig;

fn create_clippy_app(no_color: Option<bool>) -> ClippyApp<'static> {
    let clap_color_setting = if !env::var_os("NO_COLOR").is_some() && !no_color.unwrap_or(false) {
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
type AkasukiError = Box<dyn Error + Send + Sync>;
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

    let mut akasuki = Client::builder(&token)
        .event_handler(events::Handler)
        .framework(commands::create_framework(&token).await?)
        .await
        .expect("Ottotto couldn't create client >_<");

    if let Err(why) = akasuki.start_autosharded().await {
        eprintln!("Oof couldn't start akasuki T-T: {:?}", why);
    }
    Ok(())
}
