use actix_web::{App, HttpServer, web};
use dotenv::dotenv;
use sqlx::{Pool, Postgres};
use crate::config::CONFIG;
use crate::database::create_pool;
use crate::routes::routes;

pub struct AppState { pub db: Pool<Postgres>
}

pub async fn server() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let pool;

    match create_pool().await {
        Ok(conn) => {
            pool = conn
        }
        Err(err) => {
            eprintln!("Failed to create database pool: {}", err);
            std::process::exit(1);
        }
    }

    println!("🚀 Server started successfully");
    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState{
                db: pool.clone()
            }))
            .configure(routes)
    });
    server.bind(&CONFIG.server)?.run().await
}