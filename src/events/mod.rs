mod ready;

use ready::ready;
use serenity::{async_trait, client, model::prelude::Ready, prelude::*};
use tracing::instrument;
pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    #[instrument(skip_all)]
    async fn ready(&self, ctx: client::Context, info: Ready) {
        ready(ctx, info).await
    }
}
