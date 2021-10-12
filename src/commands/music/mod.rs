use poise::serenity_prelude::Mentionable;

use crate::commands::checks::guild_only;
use crate::{AkasukiContext, AkasukiResult};

pub mod play;
pub mod queue;

/// Music commands
#[poise::command(slash_command, ephemeral, check = "guild_only")]
pub async fn music(_: AkasukiContext<'_>) -> AkasukiResult<()> {
    Ok(())
}

async fn join(context: AkasukiContext<'_>) -> AkasukiResult<()> {
    let guild = context.guild().unwrap();
    let guild_id = guild.id;

    let channel_id = guild
        .voice_states
        .get(&context.author().id)
        .and_then(|voice_state| voice_state.channel_id);

    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            poise::say_reply(context, "Join a voice channel first!").await?;

            return Err(Box::new(serenity::Error::Other("User not in channel")));
        }
    };

    let manager = songbird::get(context.discord()).await.unwrap().clone();

    let (_, handler) = manager.join_gateway(guild_id, connect_to).await;

    match handler {
        Ok(connection_info) => {
            let lava_client = context.data().lavalink.clone();
            lava_client
                .create_session_with_songbird(&connection_info)
                .await?;

            poise::say_reply(context, format!("Joined {}.", connect_to.mention())).await?;
            Ok(())
        }
        Err(why) => {
            poise::say_reply(context, format!("Error joining the channel: {}.", why)).await?;
            Err(Box::new(why))
        }
    }
}
