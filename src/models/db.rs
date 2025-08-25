use sqlx::{PgPool, MySqlPool};

pub enum DbPool {
    Postgres(PgPool),
    MySql(MySqlPool),
}