use serenity::prelude::TypeMapKey;
use sqlx::PgPool;

// Defining the structures to be used for "global" data
// this data is not really global, it's just shared with Context.data
pub struct DatabasePool; // A pool of connections to the database.

impl TypeMapKey for DatabasePool {
    type Value = PgPool;
}
