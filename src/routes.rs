use actix_web::web;
use crate::handlers::health::get_health;
use crate::services::{create_user_article, fetch_user_articles, fetch_users};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/health", web::get().to(get_health))
        .service(
            web::scope("/api/v1")
                .service(
                    web::scope("/users")
                        .route("", web::get().to(fetch_users))
                        .route("/{id}/articles", web::get().to(fetch_user_articles))
                        .route("/{id}/articles", web::post().to(create_user_article))
                )
        );
}