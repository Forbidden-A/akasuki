use crate::AkasukiResult;
use clap::{
    crate_authors, crate_description, crate_name, crate_version, App as ClippyApp, AppSettings, Arg,
};
use poise::serenity_prelude::{CurrentApplicationInfo, Http};

pub mod roles;

pub async fn get_application_info(token: &str) -> AkasukiResult<CurrentApplicationInfo> {
    let http = Http::new_with_token(token);

    let info = http.get_current_application_info().await?;

    Ok(info)
}

pub fn create_clippy_app(no_color: Option<bool>) -> ClippyApp<'static> {
    let clap_color_setting = if std::env::var_os("NO_COLOR").is_none() && !no_color.unwrap_or(false)
    {
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
