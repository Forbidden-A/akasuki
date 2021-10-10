use poise::serenity_prelude::Mutex;
use sqlx::PgPool;

pub struct AkasukiData {
    pub postgres_pool: Mutex<PgPool>,
}
