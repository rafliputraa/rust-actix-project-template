use redis::Client;
use sqlx::{Pool, Postgres, Row};
use sqlx::postgres::PgPoolOptions;
use crate::config::CONFIG;
pub async fn create_pool() -> Result<Pool<Postgres>, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&CONFIG.database_url).await?;
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;

    let ping_response = sqlx::query("SELECT 1 + 1 as sum")
        .fetch_one(&pool)
        .await?;
    let sum: i32 = ping_response.get("sum");
    println!("Successfully connected to the DB, 1+1: {}", sum);
    println!("✅ Connection to the database is successful!");
    Ok(pool)
}

pub async fn create_redis_client() -> Result<Client, redis::RedisError> {
    let client = Client::open(&*CONFIG.redis_url).expect("Failed to connect to Redis");
    let mut conn = client.get_multiplexed_tokio_connection().await.expect("Failed to get Redis connection");
    let pong: String = redis::cmd("PING").query_async(&mut conn).await?;
    match pong.as_str() {
        "PONG" => {
            println!("Successfully connected to redis, PING: {}", pong);
            println!("✅ Connection to redis is successful!");
        }
        _ => println!("Redis did not respond as expected: {}", pong),
    }
    Ok(client)
}