use actix_web::web::{Data, Json, Path};
use serde::{Deserialize, Serialize};
use sqlx::{self, FromRow};
use crate::errors::ApiError;
use crate::helpers::respond_json;
use crate::server::AppState;

#[derive(Serialize, FromRow)]
pub struct User {
    id: i32,
    first_name: String,
    last_name: String,
}

#[derive(Serialize, FromRow)]
pub struct Article {
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

pub async fn fetch_users(state: Data<AppState>) -> Result<Json<Vec<User>>, ApiError>{
    let users = sqlx::query_as::<_, User>("select id, first_name, last_name from users")
        .fetch_all(&state.db)
        .await?;
    respond_json(users)
}

pub async fn fetch_user_articles(state: Data<AppState>, path: Path<i32>) -> Result<Json<Vec<Article>>, ApiError> {
    let id = path.into_inner();

    let articles = sqlx::query_as::<_, Article> (
        "select id, title, content, created_by from articles where created_by = $1"
    )
        .bind(id)
        .fetch_all(&state.db)
        .await?;
    if articles.is_empty() {
        return Err(ApiError::NotFound);
    }
    respond_json(articles)
}

pub async fn create_user_article(
    state: Data<AppState>,
    path: Path<i32>,
    body: Json<CreateArticleBody>,
) -> Result<Json<Article>, ApiError> {
    let id = path.into_inner();
   let article = sqlx::query_as::<_, Article>(
        "INSERT INTO articles (title, content, created_by) VALUES ($1, $2, $3) RETURNING id, title, content, created_by",
    )
        .bind(&body.title)
        .bind(&body.content)
        .bind(id)  // Bind id as integer
        .fetch_one(&state.db)
        .await?;
    respond_json(article)
}