use sqlx::{Database, Pool, Postgres};
use async_trait::async_trait;
use uuid::Uuid;
use sqlx::postgres::PgRow;
use sqlx::Row; 

use crate::{entities::rating::{CreateRatingResponse, ListRatingResponse, Rating}, errors::errors::InternalError};

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
    async fn list_by_merchant(&self, merchant_id: Uuid) -> Result<ListRatingResponse, InternalError>;
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

    async fn list_by_merchant(&self, merchant_id: Uuid) -> Result<ListRatingResponse, InternalError>{
      
        let query = r#"
        SELECT id, rating 
        FROM ratings 
        WHERE merchant_id = $1
     "#;

    let ratings_result = sqlx::query(query)
        .bind(merchant_id)
        .fetch_all(&self.pool) 
        .await
        .map_err(|err| InternalError {
            message: format!("Failed to fetch ratings: {}", err),
        })?;

    let ratings: Vec<Rating> = ratings_result
        .iter()
        .map(|row: &PgRow| Rating {
            rating_id: row.get("id"),
            rating: row.get("rating"),
        })
        .collect();

    Ok(ListRatingResponse { ratings })
        
    }
}