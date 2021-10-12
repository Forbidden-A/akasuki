use crate::{AkasukiContext, AkasukiResult};

pub async fn guild_only(ctx: AkasukiContext<'_>) -> AkasukiResult<bool> {
    Ok(ctx.guild_id().is_some())
}
