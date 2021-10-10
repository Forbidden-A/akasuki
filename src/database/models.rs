use sqlx::{query_as, FromRow, PgPool};

use crate::AkasukiResult;

#[derive(Debug, FromRow)]
pub struct GuildOptions {
    guild_id: i64,
    logging_enabled: bool,
    starboard_enabled: bool,
    #[sqlx(default)]
    logging_channel: Option<i64>,
    #[sqlx(default)]
    starboard_channel: Option<i64>,
}

impl GuildOptions {
    pub async fn create_for_id(id: i64, pool: &PgPool) -> AkasukiResult<GuildOptions> {
        Ok(query_as!(
            GuildOptions,
            r#"INSERT INTO akasuki.guild_options (guild_id) VALUES ($1) RETURNING *"#,
            id
        )
        .fetch_one(pool)
        .await?)
    }

    pub async fn for_id(id: i64, pool: &PgPool) -> AkasukiResult<GuildOptions> {
        Ok(query_as!(
            GuildOptions,
            r#"SELECT * FROM akasuki.guild_options WHERE guild_id = $1"#,
            id
        )
        .fetch_one(pool)
        .await?)
    }

    pub async fn for_id_or_create(id: i64, pool: &PgPool) -> AkasukiResult<GuildOptions> {
        let opts = GuildOptions::for_id(id, pool).await;
        if let Err(why) = opts {
            println!("{:?}", why);
            GuildOptions::create_for_id(id, pool).await
        } else {
            opts
        }
    }
}
