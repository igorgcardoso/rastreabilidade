use axum::{debug_handler, extract::Path, Json};

use crate::{dtos::BatchResponseDTO, errors::AppError, services::BatchService};

#[cfg(debug_assertions)]
use crate::AppState;

#[debug_handler(state = AppState)]
pub async fn find_batch_by_id(
    batch_service: BatchService,
    id: Path<i64>,
) -> Result<Json<BatchResponseDTO>, AppError> {
    let batch = batch_service.find_by_id(*id).await?;
    let batch_dto = BatchResponseDTO {
        id: batch.id().unwrap(),
        crop: batch.crop().id().unwrap(),
        classification: batch.classification().clone(),
        processing: batch.processing().clone(),
        packing: batch.packing().to_string(),
        quantity: batch.quantity(),
        tracking_code: batch.tracking_code().as_ref().unwrap().to_string(),
    };
    Ok(Json(batch_dto))
}
