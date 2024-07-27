use crate::misc::date_validation::past_or_present_validation;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Clone, Validate)]
#[serde(rename_all = "camelCase")]
pub struct BatchRequestDTO {
    pub crop_id: i64,
    pub classification: Option<String>,
    pub processing: Option<String>,
    pub packing: String,
    #[validate(range(min = 0.0))]
    pub quantity: f64,
    #[validate(custom(function = "past_or_present_validation"))]
    pub date: chrono::NaiveDate,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchResponseDTO {
    pub id: i64,
    pub crop: i64,
    pub classification: Option<String>,
    pub processing: Option<String>,
    pub packing: String,
    pub quantity: f64,
    pub tracking_code: String,
}
