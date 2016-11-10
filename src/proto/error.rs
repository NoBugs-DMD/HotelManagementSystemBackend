use std::borrow::Cow;
use rustc_serialize::json;

#[derive(Debug)]
pub enum ErrorCode {
    InvalidSchemaError = 1,
    IncompleteDataError,
    SigninError,
    SignupError,
    NotAuthorizedError,
    OldPasswordIsInvalidError,
    NotFoundError,
}

new_api_error!(InvalidSchemaError);
new_api_error!(IncompleteDataError);
new_api_error!(SigninError);
new_api_error!(SignupError);
new_api_error!(NotAuthorizedError);
new_api_error!(OldPasswordIsInvalidError);
new_api_error!(NotFoundError);

api_error_gen_from_error!(
    json::DecoderError, InvalidSchemaError
);

impl Into<i32> for ErrorCode {
    fn into(self) -> i32 {
        self as i32
    }
}

pub type ApiResult<D> = Result<D, Box<ApiError>>;

use super::response::*;
pub trait ApiError {
    fn code(&self) -> i32;
    fn description(&self) -> Cow<'static, str>;
    fn into_api_response(&self) -> ApiResponse<()> {
        ApiResponse::Err(self.code(), self.description())
    }
}