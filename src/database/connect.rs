use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::error::Error;

pub async fn connect_to_database(database_url: &str) -> Result<Pool<Postgres>, Box<dyn Error>> {
    use crate::database::migrate::migrate_database;

    let pool: Pool<Postgres> = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url).await?;

    println!("Successfully connected to database");

    match migrate_database(&pool).await {
        Ok(_) => println!("Database successfully migrated"),
        Err(e) => println!("Error while migrating database: {}", e)
    }

    return Ok(pool);
}
