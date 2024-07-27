use axum::{debug_handler, Json};
use validator::Validate;

use crate::{
    dtos::{CropRequestDTO, CropResponseDTO},
    errors::AppError,
    models::Crop,
    services::CropService,
};

#[cfg(debug_assertions)]
use crate::AppState;

#[debug_handler(state = AppState)]
pub async fn insert_crop(
    crop_service: CropService,
    body: Json<CropRequestDTO>,
) -> Result<Json<CropResponseDTO>, AppError> {
    body.validate()?;

    let crop = Crop::new(
        None,
        body.name.clone(),
        body.area,
        body.cultivation.clone(),
        body.planted_at,
        body.harvested_at,
    )?;

    let crop = crop_service.insert(&crop).await?;

    Ok(Json(CropResponseDTO {
        id: crop.id().unwrap(),
        name: crop.name().to_string(),
        area: crop.area(),
        cultivation: crop.cultivation().to_string(),
        planted_at: crop.planted_at(),
        harvested_at: *crop.harvested_at(),
    }))
}
