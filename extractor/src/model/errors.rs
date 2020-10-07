use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub struct MissingDataInHtmlError;

impl fmt::Display for MissingDataInHtmlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There's missing data in the html element")
    }
}

impl Error for MissingDataInHtmlError {
    fn description(&self) -> &str {
        "There's missing data in the html element"
    }
}

#[derive(Debug)]
pub struct DateTimeCalculationError;

impl fmt::Display for DateTimeCalculationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to perform calculatioan on a datetime struct")
    }
}

impl Error for DateTimeCalculationError {
    fn description(&self) -> &str {
        "Failed to perform calculatioan on a datetime struct"
    }
}

