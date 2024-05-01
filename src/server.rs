use actix_web::{App, HttpServer, middleware, web};
use dotenv::dotenv;
use env_logger::Builder;
use sqlx::{Pool, Postgres};
use crate::config::CONFIG;
use crate::database::create_pool;
use crate::routes::routes;
use std::io::Write;
use log::{error, info};

pub struct AppState { pub db: Pool<Postgres>
}

pub async fn server() -> std::io::Result<()> {
    dotenv().ok();

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

    let pool;

    match create_pool().await {
        Ok(conn) => {
            pool = conn
        }
        Err(err) => {
            error!("Failed to create database pool: {}", err);
            std::process::exit(1);
        }
    }

    info!("🚀 Server started successfully");
    let server = HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(AppState{
                db: pool.clone()
            }))
            .configure(routes)
    });
    server.bind(&CONFIG.server)?.run().await
}