use serde::{Deserialize, Serialize};
use chrono::NaiveDate;
#[derive(Debug, Serialize, Deserialize)]
pub struct Cn649 {
    pub serial_number: i32,
    pub draw_number1: String,
    pub draw_number2: String,
    pub draw_number3: String,
    pub draw_number4: String,
    pub draw_number5: String,
    pub draw_number6: String,
    pub draw_number7: String,
    pub draw_date: NaiveDate,
}