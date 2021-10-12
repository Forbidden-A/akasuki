use chrono::Duration;
use poise::{say_reply, serenity_prelude::Mentionable};
use std::convert::TryFrom;
use uuid::Uuid;

use crate::{AkasukiContext, AkasukiResult};

use super::join;
use crate::commands::checks::guild_only;

/// Join the channel you're currently in, and play a song.
#[poise::command(slash_command, rename = "play", check = "guild_only")]
pub async fn play_command(
    context: AkasukiContext<'_>,
    #[description = "A song URL or YouTube search query"] query: String,
) -> AkasukiResult<()> {
    let guild = context.guild().unwrap();
    let guild_id = guild.id;

    if let poise::Context::Application(ctx) = context {
        let manager = songbird::get(context.discord()).await.unwrap().clone();
        let handle = manager.get(guild_id);

        if handle.is_none() {
            join(context).await?;
        } else {
            poise::send_reply(context, |cr| {
                cr.content(format!("Searching for {}..", &query))
            })
            .await?;
        }

        let lava_client = context.data().lavalink.clone();

        let query_information = lava_client.auto_search_tracks(&query).await?;
        let uuid_tracks = Uuid::new_v4().to_string();

        if query_information.tracks.is_empty() {
            ctx.interaction
                .edit_original_interaction_response(context.discord(), |edit| {
                    edit.content("Could not find any video of the search query.")
                })
                .await?;
            return Ok(());
        } else {
            ctx.interaction
                .edit_original_interaction_response(context.discord(), |edit| {
                    edit.content("Select your song!").components(|cc| {
                        cc.create_action_row(|car| {
                            car.create_select_menu(|csm| {
                                csm.placeholder("Select a song!")
                                    .options(|c_options| {
                                        for (i, track) in
                                            query_information.tracks.iter().enumerate()
                                        {
                                            if let Some(info) = &track.info {
                                                c_options.create_option(|co| {
                                                    co.label(&info.title);
                                                    co.description(format!(
                                                        "{}. {}",
                                                        i, &info.author
                                                    ));
                                                    co.value(i)
                                                });
                                            }
                                        }
                                        c_options
                                    })
                                    .custom_id(uuid_tracks.clone())
                                    .min_values(1)
                                    .max_values(1)
                            })
                        })
                    })
                })
                .await?;
        }

        let mci = serenity::collector::CollectComponentInteraction::new(context.discord())
            .author_id(context.author().id)
            .channel_id(context.channel_id())
            .timeout(std::time::Duration::from_secs(300))
            .filter(move |mci| mci.data.custom_id == uuid_tracks)
            .await;
        if let Some(mci) = mci {
            let track = &query_information.tracks[mci.data.values[0].parse::<usize>().unwrap()];
            let content = if let Err(why) = &lava_client
                .play(guild_id, track.clone())
                .requester(context.author().id)
                .queue()
                .await
            {
                format!(
                    "Couldn't add {} to queue: {}.",
                    track.info.as_ref().unwrap().title,
                    why
                )
            } else {
                format!("Added to queue: {}.", track.info.as_ref().unwrap().title)
            };
            ctx.interaction
                .edit_original_interaction_response(context.discord(), |edit| {
                    edit.content(content).components(|cc| cc)
                })
                .await?;
        } else {
            ctx.interaction
                .edit_original_interaction_response(context.discord(), |edit| {
                    edit.content("Timed out.").components(|cc| cc)
                })
                .await?;
        }
    }

    Ok(())
}
