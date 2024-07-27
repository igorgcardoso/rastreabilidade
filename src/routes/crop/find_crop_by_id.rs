use axum::{debug_handler, extract::Path, Json};

use crate::{dtos::CropResponseDTO, errors::AppError, services::CropService};

#[cfg(debug_assertions)]
use crate::AppState;

#[debug_handler(state = AppState)]
pub async fn find_crop_by_id(
    crop_service: CropService,
    id: Path<i64>,
) -> Result<Json<CropResponseDTO>, AppError> {
    let crop = crop_service.find_by_id(*id).await?;

    Ok(Json(CropResponseDTO {
        id: crop.id().unwrap(),
        name: crop.name().to_string(),
        area: crop.area(),
        cultivation: crop.cultivation().to_string(),
        planted_at: crop.planted_at(),
        harvested_at: *crop.harvested_at(),
    }))
}
