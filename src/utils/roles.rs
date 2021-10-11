use poise::serenity_prelude as serenity;
use std::collections::HashMap;

pub fn get_non_member_roles(
    guild: &serenity::Guild,
    member: &serenity::Member,
) -> HashMap<serenity::RoleId, serenity::Role> {
    let roles = guild.roles.clone();
    roles
        .iter()
        .filter(|(id, _)| !member.roles.contains(id))
        .map(|(id, role)| (*id, role.clone()))
        .collect::<HashMap<serenity::RoleId, serenity::Role>>()
}
