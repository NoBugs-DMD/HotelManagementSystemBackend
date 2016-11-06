use rustc_serialize::json::DecoderError;
use rustc_serialize::{Encodable, Decodable};
use std::convert::From;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ErrorCode {
    OK = 0,
    InvalidSchemaError,
    SigninError,
    SignupError,
    NotAuthorizedError,
}

impl Into<i32> for ErrorCode {
    fn into(self) -> i32 {
        self as i32
    }
}

pub type ApiResult<D> = Result<D, Box<ApiError>>;

use super::response::*;
pub trait ApiError {
    fn code(&self) -> i32;
    fn description(&self) -> String;
    fn into_api_response(&self) -> ApiResponse<()>{
        ApiResponse::Err(
            self.code(),
            self.description()
        )
    }
}

macro_rules! new_api_error {
    ($ident:ident) => {
        #[derive(Debug, Clone)]
        pub struct $ident {
            description: String,
        }

        impl ::std::fmt::Display for $ident {
            fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
                write!(fmt, "description: \"{}\"", self.description)
            }
        }

        impl ::proto::error::ApiError for $ident {
            fn code(&self) -> i32 {
                ::proto::error::ErrorCode::$ident as i32
            }

            fn description(&self) -> String {
                self.description.clone()
            }
        }

        impl From<String> for $ident {
            fn from(desc: String) -> Self {
                Self {
                    description: desc
                }
            }
        }
    }
}

macro_rules! api_error_gen_from_error {
    ($from:ty, $to:ident) => {
        impl From<$from> for $to {
            fn from(raw: DecoderError) -> Self {
                $to {
                    description: format!("{}", raw)
                }
            }
        }
    }
}