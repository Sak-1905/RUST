use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use sqlx::PgPool;

mod database;
mod models;
mod routes;
mod handlers;
mod validators;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_pool = database::establish_connection().await?;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .configure(routes::config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}