// src/validators/mod.rs
use serde::Deserialize;

#[derive(Deserialize)]
pub struct NewPost {
    pub title: String,
    pub content: String,
}