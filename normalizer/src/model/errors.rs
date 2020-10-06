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