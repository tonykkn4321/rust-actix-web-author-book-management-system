# Project structure:
author_book_management_system/
├── Cargo.toml
├── .env
└── src/
    ├── main.rs
    └── config.rs
    └── models/
        └── authors.rss
        
# Start a project in current folder
cargo init

# Dependencies:
actix-web: Fast web server
dotenvy: Environment variable loader

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