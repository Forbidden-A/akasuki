use crate::{AkasukiResult, Context};
use chrono::Utc;
use poise::serenity_prelude as serenity;

const PING_URL: &'static str = "https://discord.com/api/gateway";

async fn ping_http() -> AkasukiResult<i64> {
    let start = Utc::now();
    reqwest::get(PING_URL).await?;
    let end = Utc::now();

    Ok((end - start).num_milliseconds())
}

/// Get bot ping
#[poise::command(slash_command, ephemeral)]
pub async fn ping(context: Context<'_>) -> AkasukiResult<()> {
    if let poise::Context::Application(ctx) = context {
        let interaction = ctx.interaction;
        poise::send_reply(context, |create| {
            create.content(">>> Pinging...").ephemeral(true)
        })
        .await?;
        let ack_latency = ping_http().await?;
        interaction
            .edit_original_interaction_response(context.discord(), |edit| {
                edit.content(format!(
                    "Pong!
                    >>> :file_cabinet: **ACK Latency**: ~{}ms",
                    ack_latency
                ))
            })
            .await?;
    }
    Ok(())
}
