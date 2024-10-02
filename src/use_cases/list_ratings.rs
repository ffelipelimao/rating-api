use uuid::Uuid;
use crate::{entities::rating::ListRatingResponse, errors::errors::InternalError, repository::rating_repository::RatingRepository};

pub trait ListRatingsUseCaseTrait {
    async fn list_by_merchant(&self, merchant_id: Uuid) -> Result<ListRatingResponse, InternalError>;
}

pub struct ListRatingsUseCaseImpl<R: RatingRepository> {
    repository: R,
}

impl<R: RatingRepository> ListRatingsUseCaseImpl<R> {
    pub fn new(repository: R) -> Self {
        ListRatingsUseCaseImpl { repository }
    }
}

impl<R: RatingRepository> ListRatingsUseCaseTrait for ListRatingsUseCaseImpl<R> {
    async fn list_by_merchant(&self, merchant_id: Uuid) -> Result<ListRatingResponse, InternalError>{
        return self.repository.list_by_merchant(merchant_id).await;
    }
}