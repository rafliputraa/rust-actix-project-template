use actix_web::web::{Data, Json, Path};
use serde::{Deserialize, Serialize};
use sqlx::{self, FromRow};
use crate::errors::ApiError;
use crate::helpers::respond_json;
use crate::server::AppState;
use log::debug;
use redis::AsyncCommands;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct User {
    id: i32,
    first_name: String,
    last_name: String,
}

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Article {
    id: i32,
    title: String,
    content: String,
    created_by: i32,
}

#[derive(Deserialize, Debug)]
pub struct CreateArticleBody {
    pub title: String,
    pub content: String,
}

pub async fn fetch_users(state: Data<AppState>) -> Result<Json<Vec<User>>, ApiError>{
    let users;
    let mut redis_conn = state.cache.get_multiplexed_tokio_connection().await.expect("Failed to get the redis connection");
    // Retrieve data from cache
    let cached_data: Option<String> = redis_conn.get("data:users").await.expect("Failed to retrieve the cached data");
    if let Some(data) = cached_data {
        debug!("Cached Data: {:?}", data);
        let parsed_data: Vec<User> = serde_json::from_str(&data).expect("Failed to deserialize the users data");
        users = parsed_data;
    } else {
        debug!("No data found in cache.");
        users = sqlx::query_as::<_, User>("select id, first_name, last_name from users")
            .fetch_all(&state.db)
            .await?;
        debug!("fetch_users - OUTPUT | Users: {:?}", users);
        let data_to_cache = serde_json::to_string(&users).expect("Failed to serialize the users data");
        let _: () = redis_conn.set_ex("data:users", data_to_cache, 7200).await.expect("Failed to set the users data to redis");
    }

    respond_json(users)
}

pub async fn fetch_user_articles(state: Data<AppState>, path: Path<i32>) -> Result<Json<Vec<Article>>, ApiError> {
    let id = path.into_inner();
    let articles;
    let mut redis_conn = state.cache.get_multiplexed_tokio_connection().await.expect("Failed to get redis connection");

    let cached_data: Option<String> = redis_conn.get(format!("data:articles:{}", &id)).await.expect("Failed to retrieve the cached data");
    if let Some(data) = cached_data {
        debug!("Cached Data: {:?}", data);
        let parsed_data: Vec<Article> = serde_json::from_str(&data).expect("Failed to deserialize the articles data");
        articles = parsed_data;
    } else {
        debug!("No data found in cache.");
        debug!("fetch_user_articles - INPUT | id: {}", id);
        articles = sqlx::query_as::<_, Article> (
            "select id, title, content, created_by from articles where created_by = $1"
        )
            .bind(id)
            .fetch_all(&state.db)
            .await?;

        if articles.is_empty() {
            return Err(ApiError::NotFound);
        }
        debug!("fetch_user_articles - OUTPUT | Articles: {:?}", articles);
        let data_to_cache = serde_json::to_string(&articles).expect("Failed to serialize the users data");
        let _: () = redis_conn.set_ex(format!("data:articles:{}", &id), data_to_cache, 7200).await.expect("Failed to set the articles data to redis");
    }

    respond_json(articles)
}

pub async fn create_user_article(
    state: Data<AppState>,
    path: Path<i32>,
    body: Json<CreateArticleBody>,
) -> Result<Json<Article>, ApiError> {
    let id = path.into_inner();
    let mut redis_conn = state.cache.get_multiplexed_tokio_connection().await.expect("Failed to get redis connection");

    debug!("create_user_article - INPUT | Body: {:?}", body);
   let article = sqlx::query_as::<_, Article>(
        "INSERT INTO articles (title, content, created_by) VALUES ($1, $2, $3) RETURNING id, title, content, created_by",
    )
        .bind(&body.title)
        .bind(&body.content)
        .bind(id)  // Bind id as integer
        .fetch_one(&state.db)
        .await?;
    debug!("create_user_article - OUTPUT | Article: {:?}", article);
    let _: () = redis_conn.del(format!("data:articles:{}", &id)).await.expect("Failed to delete the articles data from redis");
    respond_json(article)
}