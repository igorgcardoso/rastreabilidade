use axum::{async_trait, extract::FromRequestParts, http::request::Parts};
use sqlx::SqlitePool;

use crate::{
    errors::AppError, misc::utils::generate_token, models::Batch, repositories::BatchRepository,
    StateTrait,
};

const CODE_LENGTH: usize = 12;

pub struct BatchService {
    repository: BatchRepository,
}

impl BatchService {
    pub fn new(pool: Box<SqlitePool>) -> Self {
        Self {
            repository: BatchRepository::new(pool.clone()),
        }
    }

    async fn generate_code(&self) -> Result<String, AppError> {
        for _ in 0..500 {
            let code = generate_token(CODE_LENGTH);
            if !self.code_exists(&code).await? {
                return Ok(code);
            }
        }

        Err(AppError::BadRequest(
            "Não foi possível gerar um código".to_string(),
        ))
    }

    async fn code_exists(&self, code: &str) -> Result<bool, AppError> {
        Ok(!self
            .repository
            .find_by_tracking_code(code)
            .await?
            .is_empty())
    }

    fn validate(&self, batch: &Batch) -> Result<(), AppError> {
        if batch.date() < batch.crop().planted_at() {
            return Err(AppError::BadRequest(format!(
                "A data do lote ({}) não pode ser anterior a data da colheita do plantio ({})",
                batch.date().format("%d/%m/%Y"),
                batch.crop().harvested_at().unwrap().format("%d/%m/%Y")
            )));
        }
        Ok(())
    }

    pub async fn is_crop_in_use(&self, crop_id: i64) -> Result<bool, AppError> {
        Ok(!self.repository.find_by_crop_id(crop_id).await?.is_empty())
    }

    pub async fn list(&self) -> Result<Vec<Batch>, AppError> {
        self.repository.list().await
    }

    pub async fn find_by_id(&self, id: i64) -> Result<Batch, AppError> {
        self.repository.find_by_id(id).await
    }

    pub async fn insert(&self, mut batch: Batch) -> Result<Batch, AppError> {
        let tracking_code = self.generate_code().await?;
        batch.set_tracking_code(Some(tracking_code));
        self.validate(&batch)?;
        self.repository.insert(batch).await
    }

    pub async fn update(&self, id: i64, batch: &Batch) -> Result<Batch, AppError> {
        self.validate(batch)?;
        self.repository.update(id, batch.clone()).await
    }

    pub async fn delete(&self, id: i64) -> Result<(), AppError> {
        self.repository.delete(id).await
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for BatchService
where
    S: Send + Sync + StateTrait,
{
    type Rejection = AppError;
    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        Ok(Self::new(state.get_pool()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn get_database_pool() -> Result<SqlitePool, String> {
        Ok(SqlitePool::connect("sqlite::memory:")
            .await
            .map_err(|e| e.to_string())?)
    }

    async fn prepare_database(pool: &SqlitePool) -> Result<(), String> {
        sqlx::migrate!("./migrations")
            .run(pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn get_service(pool: Box<SqlitePool>) -> Result<BatchService, String> {
        Ok(BatchService::new(pool))
    }

    async fn init() -> Result<BatchService, String> {
        let pool = get_database_pool().await?;
        prepare_database(&pool).await?;
        get_service(Box::new(pool)).await
    }

    #[tokio::test]
    async fn generated_code_must_have_12_characters() -> Result<(), String> {
        let service = init().await?;
        for _ in 0..500 {
            let code = service.generate_code().await.unwrap();
            assert_eq!(code.len(), 12);
        }

        Ok(())
    }

    #[tokio::test]
    async fn generated_code_must_be_alphanumeric() -> Result<(), String> {
        let service = init().await?;

        let code = service.generate_code().await.unwrap();
        assert!(code.chars().all(char::is_alphanumeric));

        Ok(())
    }
}
