# Project structure:
rust-actix-api/
├── Cargo.toml
└── src/
    ├── main.rs
    ├── config.rs
    ├── models/
    │   ├── authors.rs
    │   └── books.rs
    └── routes/
        ├── authors_routes.rs
        └── books_routes.rs

# Start a project in current folder
cargo init

# Dependencies:
actix-web: Fast web server
sea-orm: Async ORM with MySQL & PostgreSQL support
dotenvy: Environment variable loader
tokio: Async runtime

# Use SeaORM CLI to generate models for local development:
cargo install sea-orm-cli
sea-orm-cli generate entity -u mysql://root:Aa161616@localhost/author_book_management_system -o src/entities
