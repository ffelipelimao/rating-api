use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use uuid::Uuid;
use crate::use_cases::list_ratings::ListRatingsUseCaseTrait;

pub struct ListRatingsHandler<U: ListRatingsUseCaseTrait> {
    use_case: U,
}

impl<U: ListRatingsUseCaseTrait> ListRatingsHandler<U> {
    pub fn new(use_case: U) -> Self {
        Self { use_case }
    }


    pub async fn list_ratings_handler(&self, Path(merchant_id): Path<Uuid>) -> impl IntoResponse {
        match self.use_case.list_by_merchant(merchant_id).await {
            Ok(response) => (StatusCode::CREATED, Json(response)).into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.message).into_response(),
        }
    }
}