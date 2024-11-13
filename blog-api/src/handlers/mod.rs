
// src/handlers/post.rs
use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::models::post::Post;
use crate::validators::NewPost;

pub async fn get_posts(db_pool: web::Data<PgPool>) -> impl Responder {
    let posts = sqlx::query_as!(Post, "SELECT * FROM posts")
        .fetch_all(db_pool.get_ref())
        .await
        .unwrap_or_else(|_| vec![]);
    HttpResponse::Ok().json(posts)
}

pub async fn get_post(web::Path(id): web::Path<i32>, db_pool: web::Data<PgPool>) -> impl Responder {
    let post = sqlx::query_as!(Post, "SELECT * FROM posts WHERE id = $1", id)
        .fetch_one(db_pool.get_ref())
        .await;

    match post {
        Ok(post) => HttpResponse::Ok().json(post),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

pub async fn create_post(new_post: web::Json<NewPost>, db_pool: web::Data<PgPool>) -> impl Responder {
    let result = sqlx::query!(
        "INSERT INTO posts (title, content, author_id) VALUES ($1, $2, $3) RETURNING id",
        new_post.title,
        new_post.content,
        1 // Assuming a static author_id for simplicity
    )
    .fetch_one(db_pool.get_ref())
    .await;

    match result {
        Ok(post) => HttpResponse::Created().json(post),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}