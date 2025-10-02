use axum::{Router, routing::get};
use sqlx::{PgPool, pool, postgres::PgPoolOptions};
use tokio;

async fn initialize_db(pool: PgPool) -> Result<String, String> {
    let create_table_query = r#"
        CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            username VARCHAR(50) NOT NULL UNIQUE,
            email VARCHAR(100) NOT NULL UNIQUE,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );
    "#;

    match sqlx::query(create_table_query).execute(&pool).await {
        Err(e) => Err(format!("Error creating users table: {}", e)),
        Ok(_) => Ok("Users table created successfully".to_string()),
    }
}

async fn root() -> &'static str {
    "Hello, World!"
}

#[tokio::main]
async fn main() {
    let pool = match PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://admin:password@localhost/db")
        .await
    {
        Err(e) => {
            eprintln!("Error connecting to the database: {}", e);
            return;
        }
        Ok(pool) => pool,
    };

    match initialize_db(pool.clone()).await {
        Err(e) => eprintln!("Error initializing the database: {}", e),
        Ok(_) => println!("Database initialized successfully"),
    };

    tracing_subscriber::fmt::init();

    let app = Router::new().route("/", get(root));

    let listener = match tokio::net::TcpListener::bind("0.0.0.0:3000").await {
        Err(e) => {
            eprintln!("Error binding to address: {}", e);
            return;
        }
        Ok(listener) => listener,
    };

    match axum::serve(listener, app).await {
        Err(e) => eprintln!("Error starting the server: {}", e),
        Ok(_) => println!("Server started successfully"),
    };
}
