use axum::{debug_handler, extract::Path, Json};

use crate::{errors::AppError, services::BatchService};

#[cfg(debug_assertions)]
use crate::AppState;

#[debug_handler(state = AppState)]
pub async fn delete_batch(
    batch_service: BatchService,
    id: Path<i64>,
) -> Result<Json<()>, AppError> {
    batch_service.delete(*id).await?;

    Ok(Json(()))
}
