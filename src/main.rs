use axum::{
    routing::post,
    Router,
};

mod handlers;
mod entities;

use crate::handlers::create_rating::create_rating_handler;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/ratings", post(create_rating_handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    
    println!("Server Running!");
    axum::serve(listener, app).await.unwrap();
}

