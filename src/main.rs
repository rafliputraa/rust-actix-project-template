mod services;

use std::error::Error;
use actix_web::{web::Data, App, HttpServer};
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres, Row};
use services::{fetch_users, create_user_article, fetch_user_articles};

pub struct AppState {
    db: Pool<Postgres>
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load config from env
    dotenv().ok();

    // Init db
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await.expect("Error building a connection pool!");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to execute the db migrations");

    let ping_response = sqlx::query("SELECT 1 + 1 as sum")
        .fetch_one(&pool)
        .await
        .expect("Error during initial query execution");
    let sum: i32 = ping_response.get("sum");
    println!("Successfully connected to the DB, 1+1: {}", sum);

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState {
                db: pool.clone()
            }))
            .service(fetch_users)
            .service(fetch_user_articles)
            .service(create_user_article)
    })
        .bind(("127.0.0.1", 8080))?
        .run().await

}
