use serenity::{client, model::prelude::Ready};
use tracing::log::info;

pub async fn ready(_ctx: client::Context, info: Ready) {
    info!("Ready as {}#{}", info.user.name, info.user.discriminator)
}
