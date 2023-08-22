use axum::Router;
use std::net::SocketAddr;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let app = static_frontend();

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
