use crate::{AkasukiResult, Context};

pub async fn guild_only(ctx: Context<'_>) -> AkasukiResult<bool> {
    Ok(ctx.guild_id().is_some())
}
