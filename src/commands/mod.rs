use serenity::{
    http::Http,
    model::{id::UserId, prelude::CurrentApplicationInfo},
};
use std::{collections::HashSet, iter::FromIterator, time::Duration};

use crate::{listeners, AkasukiError, AkasukiResult};

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

pub async fn configure<A>(
    framework: poise::FrameworkBuilder<A, AkasukiError>,
    token: &str,
) -> AkasukiResult<poise::FrameworkBuilder<A, AkasukiError>>
where
    A: std::marker::Sync + std::marker::Send,
{
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
        owners,
        listener: listeners::listener,
        ..Default::default()
    }))
}

pub async fn register<A>(
    framework: poise::FrameworkBuilder<A, AkasukiError>,
) -> AkasukiResult<poise::FrameworkBuilder<A, AkasukiError>> {
    Ok(framework)
}
