use actix_web::{web, App, HttpServer};
use dotenvy::dotenv;
use std::env;

mod db;
mod models;
mod routes;

use db::connect_db;
use routes::author::{
    list_authors, create_author, replace_author, patch_author, delete_author,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let pool = connect_db().await.expect("Failed to connect to DB");

    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/authors", web::get().to(list_authors))
            .route("/authors", web::post().to(create_author))
            .route("/authors/{id}", web::put().to(replace_author))
            .route("/authors/{id}", web::patch().to(patch_author))
            .route("/authors/{id}", web::delete().to(delete_author))
    })
    .bind(addr)?
    .run()
    .await
}
