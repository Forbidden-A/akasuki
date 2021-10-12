use crate::{AkasukiContext, AkasukiResult};

/// Boop
#[poise::command(slash_command)]
pub async fn boop(ctx: AkasukiContext<'_>) -> AkasukiResult<()> {
    let mut boop_count = 0;
    let uuid_boop = uuid::Uuid::new_v4().to_string();

    let handle = poise::send_reply(ctx, |m| {
        m.content("Boop counter!".to_string()).components(|c| {
            c.create_action_row(|ar| {
                ar.create_button(|b| {
                    b.style(poise::serenity_prelude::ButtonStyle::Primary)
                        .label("Boop me please!")
                        .emoji(poise::serenity_prelude::ReactionType::Unicode(
                            "\u{1f97a}".to_string(),
                        ))
                        .custom_id(&uuid_boop)
                })
            })
        })
    })
    .await?;

    loop {
        let mov_uuid_boop = uuid_boop.clone();
        let mci = serenity::collector::CollectComponentInteraction::new(ctx.discord())
            .author_id(ctx.author().id)
            .channel_id(ctx.channel_id())
            .timeout(std::time::Duration::from_secs(300))
            .filter(move |mci| mci.data.custom_id == mov_uuid_boop)
            .await;

        if let Some(mci) = mci {
            boop_count += 1;
            let mut msg = mci.message.clone();
            msg.edit(ctx.discord(), |m| {
                m.content(format!("Boop count: {}", boop_count))
            })
            .await?;

            mci.create_interaction_response(ctx.discord(), |ir| {
                ir.kind(poise::serenity_prelude::InteractionResponseType::DeferredUpdateMessage)
            })
            .await?;
        } else {
            handle
                .message()
                .await?
                .edit(ctx.discord(), |f| {
                    f.content(format!("Timed out! You booped me {} times!", boop_count))
                        .components(|c| {
                            c.create_action_row(|ar| {
                                ar.create_button(|b| {
                                    b.style(poise::serenity_prelude::ButtonStyle::Primary)
                                        .label("Boop me please!")
                                        .emoji(poise::serenity_prelude::ReactionType::Unicode(
                                            "\u{1f97a}".to_string(),
                                        ))
                                        .disabled(true)
                                        .custom_id(&uuid_boop)
                                })
                            })
                        })
                })
                .await?;
            break;
        }
    }

    Ok(())
}
