use serenity::{client, model::prelude::Ready};
use tracing::info;

use crate::AkasukiResult;

pub async fn ready(_ctx: &client::Context, info: &Ready) -> AkasukiResult<()> {
    info!("Ready as {}#{}", info.user.name, info.user.discriminator);
    Ok(())
}
