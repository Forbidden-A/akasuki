use sqlx::PgPool;

pub struct AkasukiData {
    pub postgres_pool: PgPool,
}
