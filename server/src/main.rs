use axum::{response::IntoResponse, routing::get, Json, Router};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqlitePool;
use std::env;
use std::net::SocketAddr;
use tower_http::services::ServeDir;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
struct Quote {
    quote_id: Uuid,
    message: String,
    created: DateTime<Utc>,
}

impl Quote {
    pub fn new(message: &str) -> Self {
        Self {
            quote_id: Uuid::new_v4(),
            message: message.to_string(),
            created: Utc::now(),
        }
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .merge(quote_routes())
        .fallback_service(static_frontend());

    let db_url: String;
    match (env::var("DATABASE_URL")) {
        Ok(url) => db_url = url,
        Err(err) => panic!(err),
    }

    let pool = SqlitePool::connect(db_url.as_str()).await.unwrap();

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    println!("--> Listening on {addr}");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn static_frontend() -> Router {
    let static_frontend_dir = ServeDir::new("../client/build");

    Router::new().nest_service("/", static_frontend_dir)
}

fn quote_routes() -> Router {
    Router::new().route("/quotes", get(quotes))
}

async fn quotes() -> impl IntoResponse {
    let quote = Quote::new("A quote from someone");
    let quote2 = Quote::new("A quote from someone else");

    let quotes: Vec<Quote> = vec![quote, quote2];

    Json(quotes)
}

// I'll need to add a handler for GET /quotes that'll handle retrieval of the list
// POST /quotes for new quotes

// Also will need a GET /quotes/{quoteId} to review and individual ID
// DELETE /quotes/{quoteId} for removing quotes
// PUT /quotes/{quoteId} for updating quotes

// I'm also going to need to add models for each of these

// Also add sqlx and get SQLite set up
