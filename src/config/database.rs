use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

pub async fn connect() -> Result<PgPool, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://user:123@localhost/rating").await?;
    
    Ok(pool)
}