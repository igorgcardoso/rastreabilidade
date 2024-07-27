use axum::{async_trait, extract::FromRequestParts, http::request::Parts};
use sqlx::SqlitePool;

use crate::{errors::AppError, models::Crop, repositories::CropRepository, StateTrait};

use super::BatchService;

pub struct CropService {
    repository: CropRepository,
    batch_service: BatchService,
}

impl CropService {
    pub fn new(pool: Box<SqlitePool>) -> Self {
        Self {
            repository: CropRepository::new(pool.clone()),
            batch_service: BatchService::new(pool),
        }
    }

    fn validate(&self, crop: &Crop) -> Result<(), AppError> {
        if let Some(harvested_at) = crop.harvested_at() {
            if *harvested_at < crop.planted_at() {
                return Err(AppError::BadRequest(format!(
                    "A data da colheita ({}) não pode ser anterior a data do plantio ({})",
                    harvested_at.format("%d/%m/%Y"),
                    crop.planted_at().format("%d/%m/%Y")
                )));
            }
        }

        Ok(())
    }

    pub async fn list(&self) -> Result<Vec<Crop>, AppError> {
        self.repository.list().await
    }

    pub async fn find_by_id(&self, id: i64) -> Result<Crop, AppError> {
        self.repository.find_by_id(id).await
    }

    pub async fn insert(&self, crop: &Crop) -> Result<Crop, AppError> {
        self.validate(crop)?;
        self.repository.insert(crop.clone()).await
    }

    pub async fn update(&self, id: i64, crop: &Crop) -> Result<Crop, AppError> {
        self.validate(crop)?;
        if self.batch_service.is_crop_in_use(id).await? {
            return Err(AppError::BadRequest(format!(
                "O plantio de ID {id} está em um lote, e portanto não pode ser alterado."
            )));
        }
        self.repository.update(id, crop.clone()).await
    }

    pub async fn delete(&self, id: i64) -> Result<(), AppError> {
        if self.batch_service.is_crop_in_use(id).await? {
            return Err(AppError::BadRequest(format!(
                "O plantio de ID {id} está em um lote, e portanto não pode ser alterado."
            )));
        }
        self.repository.delete(id).await
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for CropService
where
    S: Send + Sync + StateTrait,
{
    type Rejection = AppError;
    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        Ok(Self::new(state.get_pool()))
    }
}
