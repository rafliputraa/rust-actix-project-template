use actix_web::{
    get, post,
    web::{Data, Json, Path},
    Responder, HttpResponse
};
use serde::{Deserialize, Serialize};
use sqlx::{self, FromRow};

#[derive(Serialize, FromRow)]
struct User {
    id: i32,
    first_name: String,
    last_name: String,
}

#[derive(Serialize, FromRow)]
struct Article {
    id: i32,
    title: String,
    content: String,
    created_by: i32,
}

#[derive(Deserialize)]
pub struct CreateArticleBody {
    pub title: String,
    pub content: String,
}

#[get("/users")]
pub async fn fetch_users(state: Data<AppState>) -> impl Responder{
    match sqlx::query_as::<_, User>("select id, first_name, last_name from users")
        .fetch_all(&state.db)
        .await
    {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::NotFound().json("No users found"),
    }
}

#[get("/users/{id}/articles")]
pub async fn fetch_user_articles(state: Data<AppState>, path: Path<i32>) -> impl Responder {
    let id = path.into_inner();

    match sqlx::query_as::<_, Article> (
        "select id, title, content, created_by from articles where created_by = $1"
    )
        .bind(id)
        .fetch_all(&state.db)
        .await
    {
        Ok(articles) => HttpResponse::Ok().json(articles),
        Err(_) => HttpResponse::NotFound().json("No articles found"),
    }
}

#[post("/users/{id}/articles")]
pub async fn create_user_article(
    state: Data<AppState>,
    path: Path<i32>,
    body: Json<CreateArticleBody>,
) -> impl Responder {
    let id = path.into_inner();
    match sqlx::query_as::<_, Article>(
        "INSERT INTO articles (title, content, created_by) VALUES ($1, $2, $3) RETURNING id, title, content, created_by",
    )
        .bind(&body.title)
        .bind(&body.content)
        .bind(id)  // Bind id as integer
        .fetch_one(&state.db)
        .await
    {
        Ok(article) => HttpResponse::Ok().json(article),
        Err(_) => HttpResponse::InternalServerError().json("Failed to create user article"),
    }
}