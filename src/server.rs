use actix_web::{App, HttpServer};
use actix_web::web::Data;
use dotenv::dotenv;
use listenfd::ListenFd;
use crate::database::add_pool;

use crate::services::{create_user_article, fetch_user_articles, fetch_users};

pub async fn server() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let mut listenfd = ListenFd::from_env();

    HttpServer::new(move || {
        App::new()
            .configure(add_pool)
            .service(fetch_users)
            .service(fetch_user_articles)
            .service(create_user_article)
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0)? {
        server.listen(l)?
    } else {
        server.bind("localhost:8888")?
    };

    server.run().await
}