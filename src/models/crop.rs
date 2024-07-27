use crate::misc::date_validation::past_or_present_validation;
use chrono::NaiveDate;
use validator::{Validate, ValidationErrors};

#[derive(Debug, Clone, Validate)]
pub struct Crop {
    id: Option<i64>,
    #[validate(length(max = 255))]
    name: String,
    #[validate(range(min = 0.0))]
    area: f64,
    #[validate(length(max = 255))]
    cultivation: String,
    #[validate(custom(function = "past_or_present_validation"))]
    planted_at: NaiveDate,
    #[validate(custom(function = "past_or_present_validation"))]
    harvested_at: Option<NaiveDate>,
}

#[allow(dead_code)]
impl Crop {
    pub fn new(
        id: Option<i64>,
        name: String,
        area: f64,
        cultivation: String,
        planted_at: NaiveDate,
        harvested_at: Option<NaiveDate>,
    ) -> Result<Self, ValidationErrors> {
        let crop = Self {
            id,
            name,
            area,
            cultivation,
            planted_at,
            harvested_at,
        };

        crop.validate()?;

        Ok(crop)
    }

    pub fn id(&self) -> &Option<i64> {
        &self.id
    }

    pub fn set_id(&mut self, id: Option<i64>) {
        self.id = id;
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn area(&self) -> f64 {
        self.area
    }

    pub fn set_area(&mut self, area: f64) {
        self.area = area;
    }

    pub fn cultivation(&self) -> &str {
        &self.cultivation
    }

    pub fn set_cultivation(&mut self, cultivation: String) {
        self.cultivation = cultivation;
    }

    pub fn planted_at(&self) -> NaiveDate {
        self.planted_at
    }

    pub fn set_planted_at(&mut self, planted_at: NaiveDate) {
        self.planted_at = planted_at;
    }

    pub fn harvested_at(&self) -> &Option<NaiveDate> {
        &self.harvested_at
    }

    pub fn set_harvested_at(&mut self, harvested_at: Option<NaiveDate>) {
        self.harvested_at = harvested_at;
    }
}
