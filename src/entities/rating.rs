use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateRatingRequest {
    pub merchant_id: String,
    pub rating: f32,
}

#[derive(Serialize)]
pub struct CreateRatingResponse {
    pub rating_id: String,
    pub merchant_id: String,
    pub rating: f32,
}
