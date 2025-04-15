use serde::{Deserialize, Serialize};
use chrono::NaiveDate;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Cn649 {
    #[schema(example = 1)]
    pub serial_number: i32,
    #[schema(example = 1)]
    pub draw_number1: String,
    #[schema(example = 2)]
    pub draw_number2: String,
    #[schema(example = 3)]
    pub draw_number3: String,
    #[schema(example = 4)]
    pub draw_number4: String,
    #[schema(example = 5)]
    pub draw_number5: String,
    #[schema(example = 6)]
    pub draw_number6: String,
    #[schema(example = 7)]
    pub draw_number7: String,
    #[schema(example = "2024-01-20")]
    pub draw_date: NaiveDate,
}
