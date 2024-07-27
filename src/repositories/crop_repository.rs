use sqlx::{query, query_as, SqlitePool};

use crate::{errors::AppError, models::Crop};

#[derive(Debug)]
pub struct CropDb {
    id: i64,
    name: String,
    area: f64,
    cultivation: String,
    planted_at: chrono::NaiveDate,
    harvested_at: Option<chrono::NaiveDate>,
}

impl From<CropDb> for Crop {
    fn from(crop: CropDb) -> Self {
        Crop::new(
            Some(crop.id),
            crop.name,
            crop.area,
            crop.cultivation,
            crop.planted_at,
            crop.harvested_at,
        )
        .unwrap()
    }
}

impl From<&CropDb> for Crop {
    fn from(crop: &CropDb) -> Self {
        Crop::new(
            Some(crop.id),
            crop.name.clone(),
            crop.area,
            crop.cultivation.clone(),
            crop.planted_at,
            crop.harvested_at,
        )
        .unwrap()
    }
}

pub struct CropRepository {
    pool: Box<SqlitePool>,
}

impl CropRepository {
    pub fn new(pool: Box<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn list(&self) -> Result<Vec<Crop>, AppError> {
        let crops = query_as!(
            CropDb,
            r#"
            SELECT id, name, area, cultivation, planted_at, harvested_at
            FROM crops
            "#,
        )
        .fetch_all(&*self.pool)
        .await?;

        let crops = crops.iter().map(|crop| crop.into()).collect();

        Ok(crops)
    }

    pub async fn find_by_id(&self, id: i64) -> Result<Crop, AppError> {
        let crop = query_as!(
            CropDb,
            r#"
            SELECT id, name, area, cultivation, planted_at, harvested_at
            FROM crops
            WHERE id = ?
            "#,
            id
        )
        .fetch_optional(&*self.pool)
        .await?;

        match crop {
            Some(crop) => Ok(crop.into()),
            None => Err(AppError::NotFound(format!(
                "Plantio de ID {} não encontrado",
                id
            ))),
        }
    }

    pub async fn insert(&self, mut crop: Crop) -> Result<Crop, AppError> {
        let crop_name = crop.name().to_string();
        let crop_area = crop.area();
        let crop_cultivation = crop.cultivation().to_string();
        let crop_planted_at = crop.planted_at();
        let crop_harvested_at = crop.harvested_at();

        let crop_id = query!(
            r#"
            INSERT INTO crops (name, area, cultivation, planted_at, harvested_at)
            VALUES (?, ?, ?, ?, ?)
            RETURNING id
            "#,
            crop_name,
            crop_area,
            crop_cultivation,
            crop_planted_at,
            crop_harvested_at,
        )
        .fetch_one(&*self.pool)
        .await?;

        crop.set_id(Some(crop_id.id));

        Ok(crop)
    }

    pub async fn update(&self, id: i64, crop: Crop) -> Result<Crop, AppError> {
        let crop_name = crop.name().to_string();
        let crop_area = crop.area();
        let crop_cultivation = crop.cultivation().to_string();
        let crop_planted_at = crop.planted_at();
        let crop_harvested_at = crop.harvested_at();

        query!(
            r#"
            UPDATE crops
            SET name = ?, area = ?, cultivation = ?, planted_at = ?, harvested_at = ?
            WHERE id = ?
            "#,
            crop_name,
            crop_area,
            crop_cultivation,
            crop_planted_at,
            crop_harvested_at,
            id,
        )
        .execute(&*self.pool)
        .await?;

        Ok(crop)
    }

    pub async fn delete(&self, id: i64) -> Result<(), AppError> {
        query!(
            r#"
            DELETE FROM crops
            WHERE id = ?
            "#,
            id,
        )
        .execute(&*self.pool)
        .await?;

        Ok(())
    }
}
