use actix_web::{get, post, put, patch, delete, web, HttpResponse, Responder};
use uuid::Uuid;
use std::sync::Mutex;
use crate::models::books::Book;

pub type BookDb = Mutex<Vec<Book>>;

#[get("/books")]
pub async fn get_books(db: web::Data<BookDb>) -> impl Responder {
    let books = db.lock().unwrap();
    HttpResponse::Ok().json(&*books)
}

#[post("/books")]
pub async fn create_book(book: web::Json<Book>, db: web::Data<BookDb>) -> impl Responder {
    let mut books = db.lock().unwrap();
    books.push(book.into_inner());
    HttpResponse::Created().finish()
}

#[put("/books/{id}")]
pub async fn update_book(id: web::Path<Uuid>, book: web::Json<Book>, db: web::Data<BookDb>) -> impl Responder {
    let mut books = db.lock().unwrap();
    if let Some(b) = books.iter_mut().find(|b| b.id == *id) {
        *b = book.into_inner();
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[patch("/books/{id}")]
pub async fn patch_book(id: web::Path<Uuid>, book: web::Json<Book>, db: web::Data<BookDb>) -> impl Responder {
    let mut books = db.lock().unwrap();
    if let Some(b) = books.iter_mut().find(|b| b.id == *id) {
        if !book.title.is_empty() {
            b.title = book.title.clone();
        }
        if !book.author.is_empty() {
            b.author = book.author.clone();
        }
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[delete("/books/{id}")]
pub async fn delete_book(id: web::Path<Uuid>, db: web::Data<BookDb>) -> impl Responder {
    let mut books = db.lock().unwrap();
    let len_before = books.len();
    books.retain(|b| b.id != *id);
    if books.len() < len_before {
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::NotFound().finish()
    }
}
