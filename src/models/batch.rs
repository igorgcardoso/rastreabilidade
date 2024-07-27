use super::crop::Crop;
use crate::misc::date_validation::past_or_present_validation;
use validator::{Validate, ValidationErrors};

#[derive(Debug, Clone, Validate)]
pub struct Batch {
    id: Option<i64>,
    crop: Crop,
    #[validate(length(max = 255))]
    classification: Option<String>,
    #[validate(length(max = 255))]
    processing: Option<String>,
    #[validate(length(max = 255))]
    packing: String,
    #[validate(range(min = 0.0))]
    quantity: f64,
    tracking_code: Option<String>,
    #[validate(custom(function = "past_or_present_validation"))]
    date: chrono::NaiveDate,
}

#[allow(dead_code)]
impl Batch {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: Option<i64>,
        crop: Crop,
        classification: Option<String>,
        processing: Option<String>,
        packing: String,
        quantity: f64,
        tracking_code: Option<String>,
        date: chrono::NaiveDate,
    ) -> Result<Self, ValidationErrors> {
        let batch = Self {
            id,
            crop,
            classification,
            processing,
            packing,
            quantity,
            tracking_code,
            date,
        };
        batch.validate()?;
        Ok(batch)
    }

    pub fn id(&self) -> &Option<i64> {
        &self.id
    }

    pub fn set_id(&mut self, id: Option<i64>) {
        self.id = id;
    }

    pub fn crop(&self) -> &Crop {
        &self.crop
    }

    pub fn set_crop(&mut self, crop: Crop) {
        self.crop = crop;
    }

    pub fn classification(&self) -> &Option<String> {
        &self.classification
    }

    pub fn set_classification(&mut self, classification: Option<String>) {
        self.classification = classification;
    }

    pub fn processing(&self) -> &Option<String> {
        &self.processing
    }

    pub fn set_processing(&mut self, processing: Option<String>) {
        self.processing = processing;
    }

    pub fn packing(&self) -> &str {
        &self.packing
    }

    pub fn set_packing(&mut self, packing: String) {
        self.packing = packing;
    }

    pub fn quantity(&self) -> f64 {
        self.quantity
    }

    pub fn set_quantity(&mut self, quantity: f64) {
        self.quantity = quantity;
    }

    pub fn tracking_code(&self) -> &Option<String> {
        &self.tracking_code
    }

    pub fn set_tracking_code(&mut self, tracking_code: Option<String>) {
        self.tracking_code = tracking_code;
    }

    pub fn date(&self) -> chrono::NaiveDate {
        self.date
    }

    pub fn set_date(&mut self, date: chrono::NaiveDate) {
        self.date = date;
    }
}
