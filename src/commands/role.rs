use std::collections::HashMap;

use poise::serenity_prelude as serenity;

use crate::{AkasukiResult, Context};

/// Manage your roles
#[poise::command(slash_command, ephemeral)]
pub async fn role(ctx: Context<'_>) -> AkasukiResult<()> {
    poise::send_reply(ctx, |create| {
        create
            .content("Please use `/role <add|remove>`.")
            .ephemeral(true)
    })
    .await?;
    Ok(())
}

fn create_components_for_roles<'a>(
    create: &'a mut serenity::CreateComponents,
    _ctx: Context<'_>,
    guild: &serenity::Guild,
    user_roles: HashMap<serenity::RoleId, serenity::Role>,
    remove: bool,
    menu_id: String,
) -> &'a mut serenity::CreateComponents {
    create.create_action_row(|create_row| {
        create_row.create_select_menu(|create_menu| {
            create_menu
                .max_values(1)
                .min_values(1)
                .placeholder("Select a role")
                .options(|create_options| {
                    for (id, role) in guild.roles.iter() {
                        if (remove && !user_roles.contains_key(id))
                            || (!remove && user_roles.contains_key(id))
                        {
                            continue;
                        }
                        create_options.create_option(|create_option| {
                            let option = create_option;
                            // check for role icon; if
                            option.label(role.name.as_str()).value(id)
                        });
                    }
                    create_options
                })
                .custom_id(menu_id)
        })
    })
}

/// Manage your roles
#[poise::command(slash_command, rename = "add")]
pub async fn add_role(context: Context<'_>) -> AkasukiResult<()> {
    if let poise::Context::Application(ctx) = context {
        if let Some(guild) = context.guild() {
            let author = context.author();
            let menu_id = format!("ADD_MENU_{}", author.name);
            let mut roles = HashMap::<serenity::RoleId, serenity::Role>::new();
            {
                let guild_1 = context.guild().unwrap();
                for (id, role) in guild_1.roles.iter() {
                    if author.has_role(context.discord(), guild.id, role).await? {
                        roles.insert(*id, role.clone());
                    }
                }
            }

            let message = poise::send_reply(context, |create| {
                create
                    .content("Select a role to apply")
                    .components(|create| {
                        create_components_for_roles(create, context, &guild, roles, false, menu_id)
                    })
            })
            .await?
            .message()
            .await?;

            let interaction = message
                .await_component_interaction(&context.discord().shard)
                .await;
            if let Some(interaction) = interaction {
                let role_id = interaction.data.values[0].parse::<u64>()?;
                let role = guild.roles.get(&serenity::RoleId::from(role_id)).unwrap();
                interaction
                    .create_interaction_response(context.discord(), |f| {
                        f.interaction_response_data(|f| {
                            f.content(format!(
                                "{} Giving you {}",
                                serenity::Mentionable::mention(&author.id),
                                role
                            ))
                        })
                    })
                    .await?;
                ctx.interaction
                    .edit_original_interaction_response(context.discord(), |f| f.components(|f| f))
                    .await?;
                let mut member = interaction.member.as_ref().unwrap().clone();
                member.add_role(context.discord(), role_id).await?
            }
        } else {
            poise::send_reply(context, |create| {
                create.content("This command can only be used inside a guild.")
            })
            .await?;
        }
    }
    Ok(())
}

/// Manage your roles
#[poise::command(slash_command, rename = "remove")]
pub async fn remove_role(context: Context<'_>) -> AkasukiResult<()> {
    if let poise::Context::Application(ctx) = context {
        if let Some(guild) = context.guild() {
            let author = context.author();
            let menu_id = format!("REMOVE_MENU_{}", author.name);
            let mut roles = HashMap::<serenity::RoleId, serenity::Role>::new();
            {
                let guild_1 = context.guild().unwrap();
                for (id, role) in guild_1.roles.iter() {
                    if !author.has_role(context.discord(), guild.id, role).await? {
                        roles.insert(*id, role.clone());
                    }
                }
            }

            let message = poise::send_reply(context, |create| {
                create
                    .content("Select a role to remove")
                    .components(|create| {
                        create_components_for_roles(create, context, &guild, roles, false, menu_id)
                    })
            })
            .await?
            .message()
            .await?;

            let interaction = message
                .await_component_interaction(&context.discord().shard)
                .await;
            if let Some(interaction) = interaction {
                let role_id = interaction.data.values[0].parse::<u64>()?;
                let role = guild.roles.get(&serenity::RoleId::from(role_id)).unwrap();
                interaction
                    .create_interaction_response(context.discord(), |f| {
                        f.interaction_response_data(|f| {
                            f.content(format!(
                                "{} Taking away {}",
                                serenity::Mentionable::mention(&author.id),
                                role
                            ))
                        })
                    })
                    .await?;
                ctx.interaction
                    .edit_original_interaction_response(context.discord(), |f| f.components(|f| f))
                    .await?;
                let mut member = interaction.member.as_ref().unwrap().clone();
                member.remove_role(context.discord(), role_id).await?
            }
        } else {
            poise::send_reply(context, |create| {
                create.content("This command can only be used inside a guild.")
            })
            .await?;
        }
    }
    Ok(())
}
