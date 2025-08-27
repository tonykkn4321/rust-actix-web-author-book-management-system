use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use crate::models::authors::Author;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct NewAuthor {
    pub first_name: String,
    pub last_name: String,
}

#[derive(Deserialize)]
pub struct UpdateAuthor {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

pub async fn list_authors(pool: web::Data<PgPool>) -> HttpResponse {
    match sqlx::query_as::<_, Author>("SELECT * FROM authors")
        .fetch_all(pool.get_ref())
        .await
    {
        Ok(authors) => HttpResponse::Ok().json(authors),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn create_author(
    pool: web::Data<PgPool>,
    payload: web::Json<NewAuthor>,
) -> HttpResponse {
    match sqlx::query("INSERT INTO authors (first_name, last_name) VALUES ($1, $2)")
        .bind(&payload.first_name)
        .bind(&payload.last_name)
        .execute(pool.get_ref())
        .await
    {
        Ok(_) => HttpResponse::Created().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn replace_author(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
    payload: web::Json<NewAuthor>,
) -> HttpResponse {
    match sqlx::query("UPDATE authors SET first_name = $1, last_name = $2 WHERE id = $3")
        .bind(&payload.first_name)
        .bind(&payload.last_name)
        .bind(*path)
        .execute(pool.get_ref())
        .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn patch_author(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
    payload: web::Json<UpdateAuthor>,
) -> HttpResponse {
    let mut query = String::from("UPDATE authors SET ");
    let mut args = vec![];
    let mut param_index = 1;

    if let Some(first_name) = &payload.first_name {
        query.push_str(&format!("first_name = ${}", param_index));
        args.push(first_name.clone());
        param_index += 1;
    }

    if let Some(last_name) = &payload.last_name {
        if !args.is_empty() {
            query.push_str(", ");
        }
        query.push_str(&format!("last_name = ${}", param_index));
        args.push(last_name.clone());
        param_index += 1;
    }

    if args.is_empty() {
        return HttpResponse::BadRequest().body("No fields to update");
    }

    query.push_str(&format!(" WHERE id = ${}", param_index));

    let mut sql = sqlx::query(&query);
    for arg in &args {
        sql = sql.bind(arg);
    }
    sql = sql.bind(*path);

    match sql.execute(pool.get_ref()).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn delete_author(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
) -> HttpResponse {
    match sqlx::query("DELETE FROM authors WHERE id = $1")
        .bind(*path)
        .execute(pool.get_ref())
        .await
    {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
