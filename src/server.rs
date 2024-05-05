use actix_web::{App, HttpServer, middleware, web};
use dotenv::dotenv;
use env_logger::Builder;
use sqlx::{Pool, Postgres};
use crate::config::CONFIG;
use crate::database::{create_pool, create_redis_client};
use crate::routes::routes;
use std::io::Write;
use std::sync::Arc;
use log::{error, info};
use redis::Client;

pub struct AppState {
    pub db: Pool<Postgres>,
    pub cache: Arc<Client>,
}

pub async fn server() -> std::io::Result<()> {

    // Load the environment variables
    dotenv().ok();

    // Build the log format
    Builder::from_env(env_logger::Env::default().default_filter_or(&CONFIG.log_level))
        .format(|buf, record| {
            writeln!(
                buf,
                "[{} {}] {}",
                record.level(),
                chrono::Local::now().format("%Y-%m-%d - %H:%M:%S").to_string(),
                record.args()
            )
        })
        .init();

    // Initialize postgres pool connection
    let pool;
    match create_pool().await {
        Ok(conn) => {
            pool = conn;
        }
        Err(err) => {
            error!("Failed to create database pool: {}", err);
            std::process::exit(1);
        }
    }

    // Initialize redis client connection
    let redis_client;
    match create_redis_client().await {
        Ok(client) => {
            redis_client = client;
        }
        Err(err) => {
            error!("Failed to create redis connection: {}", err);
            std::process::exit(1);
        }
    }
    let redis_client_arc = Arc::new(redis_client);

    info!("🚀 Server started successfully");
    // Start the server
    let server = HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(AppState{
                db: pool.clone(),
                cache: redis_client_arc.clone(),
            }))
            .configure(routes)
    });
    server.bind(&CONFIG.server)?.run().await
}