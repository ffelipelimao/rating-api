use axum::{extract::Json, response::IntoResponse};
use uuid::Uuid;
use crate::entities::rating::{CreateRatingRequest, CreateRatingResponse};

pub async fn create_rating_handler(
    Json(payload): Json<CreateRatingRequest>,
) -> impl IntoResponse {   
        let response = CreateRatingResponse {
            rating_id: Uuid::new_v4().to_string(),
            order_id: payload.order_id,
            merchant_id: payload.merchant_id,
            rate: payload.rate,
        };
        (axum::http::StatusCode::CREATED, Json(response))
}