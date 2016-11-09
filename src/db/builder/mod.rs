#[macro_use]
mod macros;
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

use std::result;
type Result = result::Result<String, NoSuchPatternError>;
trait Substitute {
    fn substitute(self, pattern: &str, substitution: Option<&str>) -> Result;
}

impl Substitute for String {
    fn substitute(self, pattern: &str, substitution: Option<&str>) -> Result {
        if substitution.is_none() {
            Ok(self.replace(pattern, ""))
        } else {
            self.find(pattern)
                .ok_or(NoSuchPatternError(format!("No pattern {:?} in template", pattern)))?;
            Ok(self.replace(pattern, &substitution.unwrap()))
        }
    }
}

impl Substitute for Result {
    fn substitute(self, pattern: &str, substitution: Option<&str>) -> Result {
        match self {
            Ok(string) => string.substitute(pattern, substitution),
            Err(err) => Err(err),
        }
    }
}