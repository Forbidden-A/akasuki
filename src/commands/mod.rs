use std::{collections::HashSet, iter::FromIterator};

use serenity::{
    framework::StandardFramework,
    http::Http,
    model::{id::UserId, prelude::CurrentApplicationInfo},
};
use tracing::log::info;

use crate::AkasukiResult;

pub async fn get_application_info(token: &str) -> AkasukiResult<CurrentApplicationInfo> {
    let http = Http::new_with_token(token);

    let info = http.get_current_application_info().await?;

    Ok(info)
}

pub async fn create_framework(token: &str) -> AkasukiResult<StandardFramework> {
    let application_info = get_application_info(token).await?;
    let owners: HashSet<UserId> = if let Some(team) = application_info.team {
        HashSet::from_iter(team.members.iter().map(|m| m.user.id))
    } else {
        let mut s = HashSet::new();
        s.insert(application_info.owner.id);
        s
    };

    info!("Owners: {:?}", &owners);
    let framework = StandardFramework::new().configure(|c| c.prefix("a:").owners(owners));
    Ok(framework)
}
