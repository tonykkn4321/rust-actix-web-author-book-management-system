use actix_web::{get, post, put, patch, delete, web, HttpResponse, Responder};
use uuid::Uuid;
use std::sync::Mutex;
use crate::models::authors::Author;

pub type AuthorDb = Mutex<Vec<Author>>;

#[get("/authors")]
pub async fn get_authors(db: web::Data<AuthorDb>) -> impl Responder {
    let authors = db.lock().unwrap();
    HttpResponse::Ok().json(&*authors)
}

#[post("/authors")]
pub async fn create_author(author: web::Json<Author>, db: web::Data<AuthorDb>) -> impl Responder {
    let mut authors = db.lock().unwrap();
    authors.push(author.into_inner());
    HttpResponse::Created().finish()
}

#[put("/authors/{id}")]
pub async fn update_author(id: web::Path<Uuid>, author: web::Json<Author>, db: web::Data<AuthorDb>) -> impl Responder {
    let mut authors = db.lock().unwrap();
    if let Some(a) = authors.iter_mut().find(|a| a.id == *id) {
        *a = author.into_inner();
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[patch("/authors/{id}")]
pub async fn patch_author(id: web::Path<Uuid>, author: web::Json<Author>, db: web::Data<AuthorDb>) -> impl Responder {
    let mut authors = db.lock().unwrap();
    if let Some(a) = authors.iter_mut().find(|a| a.id == *id) {
        if !author.name.is_empty() {
            a.name = author.name.clone();
        }
        if !author.bio.is_empty() {
            a.bio = author.bio.clone();
        }
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[delete("/authors/{id}")]
pub async fn delete_author(id: web::Path<Uuid>, db: web::Data<AuthorDb>) -> impl Responder {
    let mut authors = db.lock().unwrap();
    let len_before = authors.len();
    authors.retain(|a| a.id != *id);
    if authors.len() < len_before {
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::NotFound().finish()
    }
}
