// src/routes/mod.rs
use actix_web::web;
use crate::handlers::post::{get_posts, get_post, create_post};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/posts")
            .route("", web::get().to(get_posts))
            .route("", web::post().to(create_post))
            .route("/{id}", web::get().to(get_post)),
    );
}