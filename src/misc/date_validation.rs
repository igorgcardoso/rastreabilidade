use chrono::prelude::*;
use validator::ValidationError;

pub fn past_or_present_validation(value: &NaiveDate) -> Result<(), ValidationError> {
    if value <= &Local::now().date_naive() {
        Ok(())
    } else {
        Err(ValidationError::new("date must be in the past or present"))
    }
}
