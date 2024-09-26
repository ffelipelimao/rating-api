mod handlers;
mod use_cases;     
mod repository; 
mod entities;
mod config;
mod errors;      

use axum::{
    routing::post,
    Router,
};

use std::sync::Arc;
use std::process;
use crate::handlers::create_rating::CreateRatingHandler;
use crate::use_cases::create_rating::CreateRatingUseCaseImpl;
use crate::repository::rating_repository::DBRatingRepository;
use crate::config::database::connect;


#[tokio::main]
async fn main() {

    let db_pool = match connect().await {
        Ok(pool) => pool,
        Err(e) => {
            eprintln!("Error connecting to the database: {}", e);
            process::exit(1);
        }
    };
    
    let rating_repository = DBRatingRepository::new(db_pool);
    let create_rating_use_case = CreateRatingUseCaseImpl::new(rating_repository);
    let create_rating_handler = Arc::new(CreateRatingHandler::new(create_rating_use_case));
    
    let app = Router::new().route(
        "/ratings", 
        post({
            let create_rating_handler = Arc::clone(&create_rating_handler); 
            move |json| {
                let create_rating_handler = Arc::clone(&create_rating_handler); 
                async move {
                    create_rating_handler.create_rating_handler(json).await
                }
            }
        })
    );
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server Running!");
    axum::serve(listener, app).await.unwrap();
}

