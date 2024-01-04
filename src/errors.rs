use std::{error::Error, fmt};

#[derive(Debug)]
pub struct ScaffError {
    pub message: String,
}
impl Error for ScaffError {}

impl fmt::Display for ScaffError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            fmt,
            "Scaffold did not work properly - Error: {}",
            self.message
        )
    }
}
