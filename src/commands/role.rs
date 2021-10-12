use std::collections::HashMap;

use poise::serenity_prelude::{self as serenity, Role, RoleId};

use crate::{commands::checks, utils, AkasukiResult, Context};

async fn prefix_check(ctx: Context<'_>) -> AkasukiResult<bool> {
    if let Some(guild) = ctx.guild() {
        let current_user = ctx.discord().cache.current_user();

        let permissions = guild
            .member_permissions(ctx.discord(), current_user)
            .await?;

        Ok(permissions.contains(serenity::Permissions::MANAGE_ROLES)
            || permissions.contains(serenity::Permissions::ADMINISTRATOR))
    } else {
        Ok(false)
    }
}

/// Manage your roles
#[poise::command(slash_command, ephemeral, check = "prefix_check")]
pub async fn role(ctx: Context<'_>) -> AkasukiResult<()> {
    poise::send_reply(ctx, |create| {
        create
            .content("Please use `/role <add|remove>`.")
            .ephemeral(true)
    })
    .await?;
    Ok(())
}

/// Manage your roles
#[poise::command(slash_command, rename = "add", check = "prefix_check")]
pub async fn add_role(context: Context<'_>) -> AkasukiResult<()> {
    if let Some(guild) = context.guild() {
        let uuid_menu = uuid::Uuid::new_v4().to_string();
        let mut member = guild.member(context.discord(), context.author().id).await?;
        let roles = utils::roles::get_non_member_roles(&guild, &member);
        let filtered_roles = roles
            .iter()
            .filter(|(id, _role)| {
                // TODO filter self roles.
                id.as_u64() != guild.id.as_u64()
            })
            .collect::<HashMap<&RoleId, &Role>>();
        let handle = poise::send_reply(context, |m| {
            m.content("Please select a role!");
            m.components(|c| {
                c.create_action_row(|ar| {
                    ar.create_select_menu(|menu| {
                        menu.custom_id(&uuid_menu);
                        menu.options(|options| {
                            for (id, role) in filtered_roles.iter() {
                                options.create_option(|option| {
                                    option
                                        .label(String::from(&role.name))
                                        .description("")
                                        .value(id)
                                });
                            }
                            options
                        })
                        .placeholder("Select a role")
                        .max_values(1)
                        .min_values(1)
                    })
                })
            });
            m
        })
        .await?;

        let mci = serenity::collector::CollectComponentInteraction::new(context.discord())
            .author_id(context.author().id)
            .channel_id(context.channel_id())
            .timeout(std::time::Duration::from_secs(300))
            .filter(move |mci| mci.data.custom_id == uuid_menu.clone())
            .await;
        if let Some(mci) = mci {
            let role_id = mci.data.values[0].parse::<u64>()?;
            let role = guild.roles.get(&serenity::RoleId::from(role_id)).unwrap();
            member.add_role(context.discord(), &role.id).await?;
            let mut msg = mci.message.clone();
            msg.edit(context.discord(), |m| {
                m.content(format!(
                    "Gave you: {}",
                    serenity::Mentionable::mention(&role.id)
                ))
                .components(|c| c)
            })
            .await?;

            mci.create_interaction_response(context.discord(), |ir| {
                ir.kind(poise::serenity_prelude::InteractionResponseType::DeferredUpdateMessage)
            })
            .await?;
        } else {
            handle
                .message()
                .await?
                .edit(context.discord(), |f| {
                    f.content("Timed out!").components(|c| c)
                })
                .await?;
        }
    }

    Ok(())
}

/// Manage your roles
#[poise::command(slash_command, rename = "remove", check = "checks::guild_only")]
pub async fn remove_role(context: Context<'_>) -> AkasukiResult<()> {
    if let Some(guild) = context.guild() {
    } else {
    }
    Ok(())
}
