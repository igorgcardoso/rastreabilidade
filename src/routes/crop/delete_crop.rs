use crate::{errors::AppError, services::CropService};
use axum::{debug_handler, extract::Path, Json};

#[cfg(debug_assertions)]
use crate::AppState;

#[debug_handler(state = AppState)]
pub async fn delete_crop(crop_service: CropService, id: Path<i64>) -> Result<Json<()>, AppError> {
    crop_service.delete(*id).await?;

    Ok(Json(()))
}
