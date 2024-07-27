use crate::misc::date_validation::past_or_present_validation;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Clone, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CropRequestDTO {
    pub name: String,
    pub area: f64,
    pub cultivation: String,
    #[validate(custom(function = "past_or_present_validation"))]
    pub planted_at: NaiveDate,
    #[validate(custom(function = "past_or_present_validation"))]
    pub harvested_at: Option<NaiveDate>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CropResponseDTO {
    pub id: i64,
    pub name: String,
    pub area: f64,
    pub cultivation: String,
    pub planted_at: NaiveDate,
    pub harvested_at: Option<NaiveDate>,
}
