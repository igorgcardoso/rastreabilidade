use axum::{debug_handler, Json};

use crate::{dtos::CropResponseDTO, errors::AppError, services::CropService};

#[cfg(debug_assertions)]
use crate::AppState;

#[debug_handler(state = AppState)]
pub async fn list_crops(crop_service: CropService) -> Result<Json<Vec<CropResponseDTO>>, AppError> {
    let crops = crop_service.list().await?;

    Ok(Json(
        crops
            .iter()
            .map(|crop| CropResponseDTO {
                id: crop.id().unwrap(),
                name: crop.name().to_string(),
                area: crop.area(),
                cultivation: crop.cultivation().to_string(),
                planted_at: crop.planted_at(),
                harvested_at: *crop.harvested_at(),
            })
            .collect(),
    ))
}
