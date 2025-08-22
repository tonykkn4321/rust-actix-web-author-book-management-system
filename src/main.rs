mod config;
mod models;
mod routes;

use actix_web::{web, App, HttpServer};
use crate::routes::authors_routes::{
    get_authors, create_author, update_author, patch_author, delete_author, AuthorDb,
};

use crate::routes::books_routes::{
    get_books, create_book, update_book, patch_book, delete_book, BookDb,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let author_db = web::Data::new(AuthorDb::new(Vec::new()));
    let book_db = web::Data::new(BookDb::new(Vec::new()));

    let db_url = config::get_database_url();
    println!("🔗 Connecting to database at: {}", db_url);

    HttpServer::new(move || {
        App::new()
            .app_data(author_db.clone())
            .app_data(book_db.clone())
            .service(get_authors)
            .service(create_author)
            .service(update_author)
            .service(patch_author)
            .service(delete_author)
            .service(get_books)
            .service(create_book)
            .service(update_book)
            .service(patch_book)
            .service(delete_book)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
