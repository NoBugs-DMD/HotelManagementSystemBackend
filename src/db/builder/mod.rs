#[macro_use]
mod macros;
mod delete;
mod select;
mod update;
mod insert;

pub use self::delete::DeleteQueryBuilder;
pub use self::select::SelectQueryBuilder;
pub use self::update::UpdateQueryBuilder;
pub use self::insert::InsertQueryBuilder;

pub trait QueryBuilder<'a> {
    fn default() -> Self;
    fn with_template<U>(template: U) -> Self where U: Into<Cow<'a, str>>;
    fn build(self) -> String;
}

use std::fmt;
use std::error::Error;
use std::borrow::Cow;

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
type Result<'a> = result::Result<Cow<'a, str>, NoSuchPatternError>;
trait Substitute {
    fn substitute(self, pattern: &str, substitution: Option<Cow<str>>) -> Result<'static>;
}

impl<'a> Substitute for Cow<'a, str> {
    fn substitute(self, pattern: &str, substitution: Option<Cow<str>>) -> Result<'static> {
        if substitution.is_none() {
            Ok(Cow::from(self.replace(pattern, "")))
        } else {
            self.find(pattern)
                .ok_or_else(|| NoSuchPatternError(format!("No pattern {:?} in template", pattern)))?;
            Ok(Cow::from(self.replace(pattern, &substitution.unwrap())))
        }
    }
}

impl Substitute for Result<'static> {
    fn substitute(self, pattern: &str, substitution: Option<Cow<str>>) -> Result<'static> {
        match self {
            Ok(string) => string.substitute(pattern, substitution),
            Err(err) => Err(err),
        }
    }
}