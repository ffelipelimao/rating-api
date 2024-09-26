use axum::{extract::Json, response::IntoResponse, http::StatusCode};
use crate::use_cases::create_rating::CreateRatingUseCaseTrait;
use crate::entities::rating::CreateRatingRequest;

pub struct CreateRatingHandler<U: CreateRatingUseCaseTrait> {
    use_case: U,
}

impl <U: CreateRatingUseCaseTrait> CreateRatingHandler<U>{
    pub fn new(use_case: U) -> Self {
        Self { use_case }
    }

    pub async fn create_rating_handler(&self, Json(payload): Json<CreateRatingRequest>
    ) -> impl IntoResponse {
    
        let merchant_id = payload.merchant_id;
        let rating = payload.rating;
      
        match self.use_case.create_rating(merchant_id.clone(), rating).await{
            Ok(response) => {
                (StatusCode::CREATED, Json(response)).into_response()
            }
            Err(e) => {        
                (StatusCode::INTERNAL_SERVER_ERROR, e.message).into_response()
            }
        }
    }
}


