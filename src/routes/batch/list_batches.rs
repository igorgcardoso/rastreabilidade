use axum::{debug_handler, Json};

use crate::{dtos::BatchResponseDTO, errors::AppError, services::BatchService};

#[cfg(debug_assertions)]
use crate::AppState;

#[debug_handler(state = AppState)]
pub async fn list_batches(
    batch_service: BatchService,
) -> Result<Json<Vec<BatchResponseDTO>>, AppError> {
    let batches = batch_service.list().await?;
    let batches_dto = batches
        .iter()
        .map(|batch| BatchResponseDTO {
            id: batch.id().unwrap(),
            crop: batch.crop().id().unwrap(),
            classification: batch.classification().clone(),
            processing: batch.processing().clone(),
            packing: batch.packing().to_string(),
            quantity: batch.quantity(),
            tracking_code: batch.tracking_code().as_ref().unwrap().to_string(),
        })
        .collect::<Vec<BatchResponseDTO>>();
    Ok(Json(batches_dto))
}
