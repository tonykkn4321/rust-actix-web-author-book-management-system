use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, FromRow};
use dotenvy::dotenv;
use std::env;

#[derive(Serialize, Deserialize, FromRow)]
struct Author {
    id: i32,
    first_name: String,
    last_name: String,
}

#[derive(Deserialize)]
struct NewAuthor {
    first_name: String,
    last_name: String,
}

async fn get_authors(pool: web::Data<PgPool>) -> impl Responder {
    let authors = sqlx::query_as::<_, Author>("SELECT * FROM authors")
        .fetch_all(pool.get_ref())
        .await
        .unwrap_or_default();
    HttpResponse::Ok().json(authors)
}

async fn create_author(
    pool: web::Data<PgPool>,
    new_author: web::Json<NewAuthor>,
) -> impl Responder {
    let _ = sqlx::query("INSERT INTO authors (first_name, last_name) VALUES ($1, $2)")
        .bind(&new_author.first_name)
        .bind(&new_author.last_name)
        .execute(pool.get_ref())
        .await;
    HttpResponse::Created().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&db_url).await.expect("Failed to connect to DB");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/authors", web::get().to(get_authors))
            .route("/authors", web::post().to(create_author))
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}
