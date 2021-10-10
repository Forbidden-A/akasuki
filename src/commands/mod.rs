mod ping;
mod role;
use serenity::{
    http::Http,
    model::{id::UserId, prelude::CurrentApplicationInfo},
};
use std::{collections::HashSet, iter::FromIterator, time::Duration};

use crate::{global_data::AkasukiData, listeners, AkasukiError, AkasukiResult, Context};

pub async fn get_application_info(token: &str) -> AkasukiResult<CurrentApplicationInfo> {
    let http = Http::new_with_token(token);

    let info = http.get_current_application_info().await?;

    Ok(info)
}

pub async fn on_error<D>(e: AkasukiError, ctx: poise::ErrorContext<'_, D, AkasukiError>) {
    println!("Encountered an error: {:?}", e);
    match ctx {
        poise::ErrorContext::Listener(event) => {
            println!("Error in listener while processing {:?}: {}", event, e)
        }
        poise::ErrorContext::Setup => println!("Setup failed: {}", e),
        _ => (),
    }
}

// Register application commands in this guild or globally
///
/// Run with no arguments to register in guild, run with argument "global" to register globally.
#[poise::command(prefix_command, hide_in_help, rename = "register")]
async fn register_cmd(ctx: Context<'_>, #[flag] global: bool) -> AkasukiResult<()> {
    poise::samples::register_application_commands(ctx, global).await?;

    Ok(())
}

pub async fn configure(
    framework: poise::FrameworkBuilder<AkasukiData, AkasukiError>,
    token: &str,
) -> AkasukiResult<poise::FrameworkBuilder<AkasukiData, AkasukiError>> {
    let application_info = get_application_info(token).await?;
    let owners: HashSet<UserId> = if let Some(team) = application_info.team {
        HashSet::from_iter(team.members.iter().map(|m| m.user.id))
    } else {
        let mut s = HashSet::new();
        s.insert(application_info.owner.id);
        s
    };

    Ok(framework.options(poise::FrameworkOptions {
        on_error: |error, ctx| Box::pin(on_error(error, ctx)),
        prefix_options: poise::PrefixFrameworkOptions {
            edit_tracker: Some(poise::EditTracker::for_timespan(Duration::from_secs(3600))),
            ..Default::default()
        },
        application_options: poise::ApplicationFrameworkOptions {
            ..Default::default()
        },
        owners,
        listener: listeners::listener,
        ..Default::default()
    }))
}

pub async fn register(
    framework: poise::FrameworkBuilder<AkasukiData, AkasukiError>,
) -> AkasukiResult<poise::FrameworkBuilder<AkasukiData, AkasukiError>> {
    Ok(framework
        .command(ping::ping(), |f| f.category("General"))
        .command(role::role(), |f| {
            f.category("Utility")
                .subcommand(role::add_role(), |f| f.category("Utility"))
                .subcommand(role::remove_role(), |f| f.category("Utility"))
        })
        .command(register_cmd(), |f| f.category("Owner Only")))
}
