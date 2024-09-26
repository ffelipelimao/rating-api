use sqlx::{Database, Pool, Postgres};
use async_trait::async_trait;
use uuid::Uuid;

use crate::{entities::rating::CreateRatingResponse, errors::errors::InternalError};

pub struct DBRatingRepository<DB: Database> {
    pool: Pool<DB>,
}

impl<DB: Database> DBRatingRepository<DB> {
    pub fn new(pool: Pool<DB>) -> Self {
        DBRatingRepository { pool }
    }
}

#[async_trait]
pub trait RatingRepository {
    async fn create_rating(&self, merchant_id: Uuid, rating: f32) -> Result<CreateRatingResponse, InternalError>;
}

#[async_trait]
impl RatingRepository for DBRatingRepository<Postgres> {
    async fn create_rating(&self, merchant_id: Uuid, rating: f32) -> Result<CreateRatingResponse, InternalError> {
        let rating_id = Uuid::new_v4();
        
        let result = sqlx::query(
            "INSERT INTO ratings (id, merchant_id, rating) VALUES ($1, $2, $3)"
        )
        .bind(rating_id) //TODO formartar para double com duas casas decimais só
        .bind(merchant_id)
        .bind(rating)
        .execute(&self.pool) 
        .await;
    
        match result {
            Ok(_) => Ok(
                CreateRatingResponse {
                    rating_id: rating_id.to_string(), 
                    merchant_id: merchant_id.to_string(),
                    rating
                }
            ),
            Err(err) => Err(InternalError::new(err)),
        }
    }
}