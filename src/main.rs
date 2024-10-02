mod handlers;
mod use_cases;
mod repository;
mod entities;
mod config;
mod errors;

use axum::{
    extract::{Extension, Path},
    routing::{get, post},
    Json, Router,
    response::IntoResponse,
};
use std::sync::Arc;
use std::process;
use uuid::Uuid;

use crate::config::database::connect;
use crate::handlers::create_rating::CreateRatingHandler;
use crate::handlers::list_ratings::ListRatingsHandler;
use crate::repository::rating_repository::DBRatingRepository;
use crate::use_cases::create_rating::CreateRatingUseCaseImpl;
use crate::use_cases::list_ratings::ListRatingsUseCaseImpl;

#[tokio::main]
async fn main() {
   
    let db_pool = match connect().await {
        Ok(pool) => pool,
        Err(e) => {
            eprintln!("Erro ao conectar ao banco de dados: {}", e);
            process::exit(1);
        }
    };

    let rating_repository = DBRatingRepository::new(db_pool);

    let create_rating_use_case = CreateRatingUseCaseImpl::new(rating_repository);
    let create_rating_handler = Arc::new(CreateRatingHandler::new(create_rating_use_case));

    // let list_rating_use_case = ListRatingsUseCaseImpl::new(rating_repository);
    // let list_ratings_handler = Arc::new(ListRatingsHandler::new(list_rating_use_case));

    let app = Router::new()
    .route(
        "/ratings",
        post({
            let create_rating_handler = Arc::clone(&create_rating_handler);
            move |json| {
                let create_rating_handler = Arc::clone(&create_rating_handler);
                async move {
                    create_rating_handler.create_rating_handler(json).await
                }
            }
        }),
    );
   /*  .route(
        "/ratings/:merchant_id",
        get({
            let list_ratings_handler = Arc::clone(&list_ratings_handler);
            move |Path(merchant_id): Path<Uuid>| {
                let handler = Arc::clone(&list_ratings_handler);
                async move {
                    handler.list_ratings_handler(Path(merchant_id)).await
                }
            }
        }),
    );
    */

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Servidor rodando na porta 3000!");
    axum::serve(listener, app).await.unwrap();
}
