use actix_web::{web, App, HttpServer, HttpResponse};
use sqlx::{MySqlPool, PgPool};
use std::env;

mod config;
mod models;
use models::{authors::Author, books::Book};

async fn get_authors(pool: web::Data<MySqlPool>) -> HttpResponse {
    let authors = sqlx::query_as::<_, Author>("SELECT * FROM authors")
        .fetch_all(pool.get_ref())
        .await;

    match authors {
        Ok(authors) => HttpResponse::Ok().json(authors),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn create_author(pool: web::Data<MySqlPool>, new_author: web::Json<Author>) -> HttpResponse {
    let result = sqlx::query("INSERT INTO authors (first_name, last_name) VALUES (?, ?)")
        .bind(&new_author.first_name)
        .bind(&new_author.last_name)
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Created().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// Similar functions for books...

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    config::init();
    let database_url = config::get_database_url();
    let app_env = config::get_app_env();

    let pool = if app_env == "production" {
        PgPool::connect(&database_url).await.expect("Failed to create pool")
    } else {
        MySqlPool::connect(&database_url).await.expect("Failed to create pool")
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/authors/", web::get().to(get_authors))
            .route("/authors/", web::post().to(create_author))
            // Add routes for books...
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}