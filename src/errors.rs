use std::{error::Error, fmt};

#[derive(Debug)]
pub struct ScaffError {
    pub message: String,
}
impl Error for ScaffError {}

impl fmt::Display for ScaffError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "Could not get device id - Error: {}", self.message)
    }
}
