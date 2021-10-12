use std::ops::Add;

use poise::{
    say_reply,
    serenity_prelude::{Colour, Mentionable, UserId},
};
use rand::Rng;

use crate::{AkasukiContext, AkasukiResult};

use super::guild_only;

/// Get the current song queue.
#[poise::command(slash_command, rename = "queue", check = "guild_only")]
pub async fn queue_command(context: AkasukiContext<'_>) -> AkasukiResult<()> {
    let guild = context.guild().unwrap();
    let guild_id = guild.id;

    if let poise::Context::Application(ctx) = context {
        let manager = songbird::get(context.discord()).await.unwrap().clone();

        let handle = manager.get(guild_id);

        if handle.is_none() {
            poise::send_reply(context, |cr| cr.content("Not playing anything!")).await?;
        } else {
            poise::send_reply(context, |cr| cr.content("Getting queue!")).await?;
        }

        let lava_client = context.data().lavalink.clone();

        let nodes = lava_client.nodes().await;

        let node = nodes.get(guild_id.as_u64());

        if node.is_none() {
            eprintln!("node is none");
            return Ok(());
        } else {
            let node = &node.unwrap();
            let queue = &node.queue;
            let mut c = String::new();
            let now_playing = &node.now_playing;
            for (i, tq) in queue.iter().enumerate() {
                if let Some(info) = &tq.track.info {
                    c.push_str(format!("`{}. {}`", i + 1, info.title).as_str());
                    if let Some(user) = tq.requester {
                        c.push_str(
                            format!(" - Requested by {}", UserId::from(*user.as_u64()).mention())
                                .as_str(),
                        )
                    }
                    if let Some(np) = now_playing {
                        if let Some(np_info) = &np.track.info {
                            if info.identifier == np_info.identifier {
                                c.push_str(format!(" - Now Playing!").as_str())
                            }
                        }
                    }
                    c.push('\n');
                }
            }
            ctx.interaction
                .edit_original_interaction_response(context.discord(), |edit| {
                    edit.create_embed(|e| {
                        let mut rng = rand::thread_rng();
                        e.title("Current queue:")
                            .description(c)
                            .colour(Colour::from(rng.gen_range(0x0..0xFFFFFF)))
                    })
                    .content("")
                })
                .await?;
        }
    }

    Ok(())
}
