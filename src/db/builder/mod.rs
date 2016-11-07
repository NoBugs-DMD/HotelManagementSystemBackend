mod delete;
mod select;

pub use self::delete::*;
pub use self::select::*;

pub trait QueryBuilder {
    fn new() -> Self;
    fn with_template(template: String) -> Self;
    fn build(self) -> String;
}

use std::fmt;
use std::error::Error;

#[derive(Debug)]
struct NoSuchPatternError(String);

impl fmt::Display for NoSuchPatternError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(&self.0)
    }
} 

impl Error for NoSuchPatternError {
    fn description(&self) -> &str {
        &self.0
    }
}

fn substitute(pattern: &str, target: String, substitution: Option<&String>) -> Result<String, NoSuchPatternError> {
    if substitution.is_none() {
        Ok(target.replace(pattern, ""))
    } else {
        target.find(pattern).ok_or(NoSuchPatternError(format!("No pattern {:?} in template", pattern)))?;
        Ok(target.replace(pattern, &substitution.unwrap()))
    }
}
