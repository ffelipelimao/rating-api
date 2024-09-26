
use std::str::FromStr;

use uuid::Uuid;
use crate::{entities::rating::CreateRatingResponse, errors::errors::InternalError, repository::rating_repository::RatingRepository};

pub trait CreateRatingUseCaseTrait {
    async fn create_rating(&self, merchant_id: String, rating: f32) -> Result<CreateRatingResponse, InternalError>;
}

pub struct CreateRatingUseCaseImpl<R: RatingRepository> {
    repository: R,
}

impl<R: RatingRepository> CreateRatingUseCaseImpl<R> {
    pub fn new(repository: R) -> Self {
        CreateRatingUseCaseImpl { repository }
    }
}

impl<R: RatingRepository> CreateRatingUseCaseTrait for CreateRatingUseCaseImpl<R> {
    async fn create_rating(&self, merchant_id: String, rating: f32) -> Result<CreateRatingResponse, InternalError> {

        let merchant_uuid = match Uuid::from_str(&merchant_id) {
            Ok(merchant_uuid) => merchant_uuid,
            Err(err) => {
                return Err(InternalError::new(err)); //TODO isso deveria ser um bad request
            }
        };
        
        self.repository.create_rating(merchant_uuid, rating).await
    }
}