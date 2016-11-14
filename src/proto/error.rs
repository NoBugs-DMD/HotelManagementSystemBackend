use rustc_serialize::json;
use std::error::Error;
use std::fmt::Display;
use iron::prelude::*;
use hyper::status::StatusCode;

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

api_error_gen_from_error!(json::DecoderError, InvalidSchemaError);

impl Into<i32> for ErrorCode {
    fn into(self) -> i32 {
        self as i32
    }
}

pub trait ApiError: Error + Send {
    fn code(&self) -> i32;
    fn json(&self) -> String {
        format!("{{\"err_code\":\"{}\", \"description\":\"{}\"}}",
                self.code(),
                self.description())
    }
}

pub type ApiResult<D> = Result<D, Box<ApiError>>;

impl Error for Box<ApiError> {
    fn description(&self) -> &str {
        Error::description(&**self)
    }
}

impl From<Box<ApiError>> for IronError {
    fn from(err: Box<ApiError>) -> IronError {
        let mut response = Response::with(StatusCode::Forbidden);
        response.body = Some(box err.json());

        IronError {
            error: box err,
            response: response,
        }
    }
}
