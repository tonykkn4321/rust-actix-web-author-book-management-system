# Project structure:
rust-actix-author-book-management-system/
├── Cargo.toml
├── .env (for localhost use)
└── src/
    ├── main.rs
    ├── db.rs
    ├── models/
    │   ├── mod.rs
    │   └── author.rs
    └── routes/
        ├── mod.rs
        └── author.rs

        
# Start a project in current folder
cargo init

# Dependencies:
actix-web: Fast web server
dotenvy: Environment variable loader
sqlx = { version = "0.6", features = ["mysql", "runtime-tokio-rustls"] } 
sqlx = { version = "0.6", features = ["postgres", "runtime-tokio-rustls"] } 
# Run the server:
cargo run


# For MySQL:
CREATE TABLE authors (
    id INT AUTO_INCREMENT PRIMARY KEY,
    first_name VARCHAR(100) NOT NULL,
    last_name VARCHAR(100) NOT NULL
);

CREATE TABLE books (
    id INT AUTO_INCREMENT PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    year INT NOT NULL CHECK (year > 0),
    author_id INT,
    FOREIGN KEY (author_id) REFERENCES authors(id) ON DELETE CASCADE
);

# For PostgreSQL:
CREATE TABLE authors (
    id SERIAL PRIMARY KEY,
    first_name VARCHAR(100) NOT NULL,
    last_name VARCHAR(100) NOT NULL
);

CREATE TABLE books (
    id SERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    year INT NOT NULL CHECK (year > 0),
    author_id INT,
    FOREIGN KEY (author_id) REFERENCES authors(id) ON DELETE CASCADE
);

# Railways:
cargo build --release
cargo run --release