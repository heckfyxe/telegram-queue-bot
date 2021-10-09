use once_cell::sync::OnceCell;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

pub static DATABASE: OnceCell<PgPool> = OnceCell::new();

pub async fn initialize() {
    if DATABASE.get().is_some() {
        return;
    }

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&dotenv::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    DATABASE.set(pool).unwrap();
}
