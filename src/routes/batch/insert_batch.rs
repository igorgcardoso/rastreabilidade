use axum::{debug_handler, Json};
use validator::Validate;

use crate::{
    dtos::{BatchRequestDTO, BatchResponseDTO},
    errors::AppError,
    models::Batch,
    services::{BatchService, CropService},
};

#[cfg(debug_assertions)]
use crate::AppState;

#[debug_handler(state = AppState)]
pub async fn insert_batch(
    batch_service: BatchService,
    crop_service: CropService,
    body: Json<BatchRequestDTO>,
) -> Result<Json<BatchResponseDTO>, AppError> {
    body.validate()?;

    let crop = crop_service.find_by_id(body.crop_id).await?;

    let batch = Batch::new(
        None,
        crop,
        body.classification.clone(),
        body.processing.clone(),
        body.packing.clone(),
        body.quantity,
        None,
        body.date,
    )?;

    let batch = batch_service.insert(batch).await?;

    Ok(Json(BatchResponseDTO {
        id: batch.id().unwrap(),
        crop: batch.crop().id().unwrap(),
        classification: batch.classification().clone(),
        processing: batch.processing().clone(),
        packing: batch.packing().to_string(),
        quantity: batch.quantity(),
        tracking_code: batch.tracking_code().as_ref().unwrap().to_string(),
    }))
}
