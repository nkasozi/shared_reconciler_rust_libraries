use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Default, Validate, Serialize, PartialEq, Clone, Eq, Deserialize, Debug)]
pub struct FileRow {
    #[validate(length(min = 1, message = "please data in the file row"))]
    pub raw_data: String,
    pub row_number: u64,
}
