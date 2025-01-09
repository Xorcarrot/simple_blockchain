use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct MaxAttemptsError;

impl Error for MaxAttemptsError {}

impl Display for MaxAttemptsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Maximum number of attempts reached")
    }
}