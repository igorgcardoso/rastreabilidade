use sqlx::{query, SqlitePool};

use crate::{
    errors::AppError,
    models::{Batch, Crop},
};

pub struct BatchRepository {
    pool: Box<SqlitePool>,
}

impl BatchRepository {
    pub fn new(pool: Box<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn list(&self) -> Result<Vec<Batch>, AppError> {
        let batches = query!(
            r#"
            SELECT b.id, b.crop_id, b.classification, b.processing, b.packing, b.quantity, b.tracking_code, b.date, c.name as crop_name, c.area as crop_area, c.cultivation as crop_cultivation, c.planted_at as crop_planted_at, c.harvested_at as crop_harvested_at
            FROM batches b
            INNER JOIN crops c ON b.crop_id = c.id;
            "#,
        )
        .fetch_all(&*self.pool)
        .await?;

        let batches = batches
            .iter()
            .map(|batch| {
                Batch::new(
                    Some(batch.id),
                    Crop::new(
                        Some(batch.crop_id),
                        batch.crop_name.clone(),
                        batch.crop_area,
                        batch.crop_cultivation.clone(),
                        batch.crop_planted_at,
                        batch.crop_harvested_at,
                    )
                    .unwrap(),
                    batch.classification.clone(),
                    batch.processing.clone(),
                    batch.packing.clone(),
                    batch.quantity,
                    Some(batch.tracking_code.clone()),
                    batch.date,
                )
                .unwrap()
            })
            .collect();

        Ok(batches)
    }

    pub async fn find_by_id(&self, id: i64) -> Result<Batch, AppError> {
        let batch = query!(
            r#"
            SELECT b.id, b.crop_id, b.classification, b.processing, b.packing, b.quantity, b.tracking_code, b.date, c.name as crop_name, c.area as crop_area, c.cultivation as crop_cultivation, c.planted_at as crop_planted_at, c.harvested_at as crop_harvested_at
            FROM batches b
            INNER JOIN crops c ON b.crop_id = c.id
            WHERE b.id = ?;
            "#,
            id
        )
        .fetch_one(&*self.pool)
        .await?;

        let crop = Crop::new(
            Some(batch.crop_id),
            batch.crop_name,
            batch.crop_area,
            batch.crop_cultivation,
            batch.crop_planted_at,
            batch.crop_harvested_at,
        )?;

        Ok(Batch::new(
            Some(batch.id),
            crop,
            batch.classification,
            batch.processing,
            batch.packing,
            batch.quantity,
            Some(batch.tracking_code),
            batch.date,
        )?)
    }

    pub async fn find_by_tracking_code<S: ToString>(
        &self,
        code: S,
    ) -> Result<Vec<Batch>, AppError> {
        let code = code.to_string();
        let batches = query!(
            r#"
            SELECT b.id, b.crop_id, b.classification, b.processing, b.packing, b.quantity, b.tracking_code, b.date, c.name as crop_name, c.area as crop_area, c.cultivation as crop_cultivation, c.planted_at as crop_planted_at, c.harvested_at as crop_harvested_at
            FROM batches b
            INNER JOIN crops c ON b.crop_id = c.id
            WHERE b.tracking_code = ?;
            "#,
            code,
        )
        .fetch_all(&*self.pool)
        .await?;

        let batches = batches
            .iter()
            .map(|batch| {
                Batch::new(
                    Some(batch.id),
                    Crop::new(
                        Some(batch.crop_id),
                        batch.crop_name.clone(),
                        batch.crop_area,
                        batch.crop_cultivation.clone(),
                        batch.crop_planted_at,
                        batch.crop_harvested_at,
                    )
                    .unwrap(),
                    batch.classification.clone(),
                    batch.processing.clone(),
                    batch.packing.clone(),
                    batch.quantity,
                    Some(batch.tracking_code.clone()),
                    batch.date,
                )
                .unwrap()
            })
            .collect();

        Ok(batches)
    }

    pub async fn find_by_crop_id(&self, crop_id: i64) -> Result<Vec<Batch>, AppError> {
        let batches = query!(
            r#"
            SELECT b.id, b.crop_id, b.classification, b.processing, b.packing, b.quantity, b.tracking_code, b.date, c.name as crop_name, c.area as crop_area, c.cultivation as crop_cultivation, c.planted_at as crop_planted_at, c.harvested_at as crop_harvested_at
            FROM batches b
            INNER JOIN crops c ON b.crop_id = c.id
            WHERE b.crop_id = ?;
            "#,
            crop_id
        )
        .fetch_all(&*self.pool)
        .await?;

        let batches = batches
            .iter()
            .map(|batch| {
                Batch::new(
                    Some(batch.id),
                    Crop::new(
                        Some(batch.crop_id),
                        batch.crop_name.clone(),
                        batch.crop_area,
                        batch.crop_cultivation.clone(),
                        batch.crop_planted_at,
                        batch.crop_harvested_at,
                    )
                    .unwrap(),
                    batch.classification.clone(),
                    batch.processing.clone(),
                    batch.packing.clone(),
                    batch.quantity,
                    Some(batch.tracking_code.clone()),
                    batch.date,
                )
                .unwrap()
            })
            .collect();

        Ok(batches)
    }

    pub async fn insert(&self, mut batch: Batch) -> Result<Batch, AppError> {
        let crop_id = batch.crop().id().unwrap();
        let classification = batch.classification().clone();
        let processing = batch.processing().clone();
        let packing = batch.packing();
        let quantity = batch.quantity();
        let tracking_code = batch.tracking_code().clone().unwrap();
        let date = batch.date();

        let id = query!(
            r#"
            INSERT INTO batches (crop_id, classification, processing, packing, quantity, tracking_code, date)
            VALUES (?, ?, ?, ?, ?, ?, ?);
            "#,
            crop_id,
            classification,
            processing,
            packing,
            quantity,
            tracking_code,
            date
        )
        .execute(&*self.pool)
        .await?
        .last_insert_rowid();

        batch.set_id(Some(id));

        Ok(batch)
    }

    pub async fn update(&self, id: i64, batch: Batch) -> Result<Batch, AppError> {
        let crop_id = batch.crop().id().unwrap();
        let classification = batch.classification().clone();
        let processing = batch.processing().clone();
        let packing = batch.packing();
        let quantity = batch.quantity();
        let tracking_code = batch.tracking_code().clone().unwrap();
        let date = batch.date();

        query!(
            r#"
            UPDATE batches
            SET crop_id = ?, classification = ?, processing = ?, packing = ?, quantity = ?, tracking_code = ?, date = ?
            WHERE id = ?;
            "#,
            crop_id,
            classification,
            processing,
            packing,
            quantity,
            tracking_code,
            date,
            id
        )
        .execute(&*self.pool)
        .await?;

        Ok(batch)
    }

    pub async fn delete(&self, id: i64) -> Result<(), AppError> {
        query!(
            r#"
            DELETE FROM batches
            WHERE id = ?;
            "#,
            id
        )
        .execute(&*self.pool)
        .await?;

        Ok(())
    }
}
