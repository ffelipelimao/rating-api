use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateRatingRequest {
    pub order_id: String,
    pub merchant_id: String,
    pub rate: f32,
}

#[derive(Serialize)]
pub struct CreateRatingResponse {
    pub rating_id: String,
    pub order_id: String,
    pub merchant_id: String,
    pub rate: f32,
}
