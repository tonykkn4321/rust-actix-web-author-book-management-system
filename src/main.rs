use actix_web::{web, App, HttpServer, HttpResponse};
use sqlx::{PgPool, MySqlPool};
use std::env;

mod config;
mod models;
use models::{authors::Author, books::Book, db::DbPool};

async fn get_authors(pool: &DbPool) -> HttpResponse {
    match pool {
        DbPool::Postgres(pg_pool) => {
            match sqlx::query_as::<_, Author>("SELECT * FROM authors")
                .fetch_all(pg_pool)
                .await
            {
                Ok(authors) => HttpResponse::Ok().json(authors),
                Err(err) => {
                    eprintln!("Error fetching authors: {:?}", err);
                    HttpResponse::InternalServerError().finish()
                }
            }
        }
        DbPool::MySql(mysql_pool) => {
            match sqlx::query_as::<_, Author>("SELECT * FROM authors")
                .fetch_all(mysql_pool)
                .await
            {
                Ok(authors) => HttpResponse::Ok().json(authors),
                Err(err) => {
                    eprintln!("Error fetching authors: {:?}", err);
                    HttpResponse::InternalServerError().finish()
                }
            }
        }
    }
}

async fn create_author(pool: &DbPool, new_author: web::Json<Author>) -> HttpResponse {
    match pool {
        DbPool::Postgres(pg_pool) => {
            match sqlx::query("INSERT INTO authors (first_name, last_name) VALUES (?, ?)")
                .bind(&new_author.first_name)
                .bind(&new_author.last_name)
                .execute(pg_pool)
                .await
            {
                Ok(_) => HttpResponse::Created().finish(),
                Err(err) => {
                    eprintln!("Error creating author: {:?}", err);
                    HttpResponse::InternalServerError().finish()
                }
            }
        }
        DbPool::MySql(mysql_pool) => {
            match sqlx::query("INSERT INTO authors (first_name, last_name) VALUES (?, ?)")
                .bind(&new_author.first_name)
                .bind(&new_author.last_name)
                .execute(mysql_pool)
                .await
            {
                Ok(_) => HttpResponse::Created().finish(),
                Err(err) => {
                    eprintln!("Error creating author: {:?}", err);
                    HttpResponse::InternalServerError().finish()
                }
            }
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    config::init();
    let database_url = config::get_database_url();
    let app_env = config::get_app_env();

    let pool: DbPool = if app_env == "production" {
        DbPool::Postgres(PgPool::connect(&database_url).await.expect("Failed to create PostgreSQL pool"))
    } else {
        DbPool::MySql(MySqlPool::connect(&database_url).await.expect("Failed to create MySQL pool"))
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/authors/", web::get().to(get_authors))
            .route("/authors/", web::post().to(create_author))
    })
    .bind("0.0.0.0:8080")? // Binding to all interfaces
    .run()
    .await
}