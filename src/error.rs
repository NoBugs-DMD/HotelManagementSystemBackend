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
}

impl Into<i32> for ErrorCode {
    fn into(self) -> i32 {
        self as i32
    }
}

use ::response::*;
pub trait ApiError: Error + Encodable + Into<ApiResponse<()>> {
    fn code() -> i32;
}

macro_rules! new_api_error {
    ($ident:ident) => {
        #[derive(Debug, Clone, RustcEncodable)]
        pub struct $ident {
            description: String,
        }

        api_error_into_api_response!($ident);

        impl ::std::fmt::Display for $ident {
            fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
                write!(fmt, "description: \"{}\"", self.description)
            }
        }

        impl ::error::ApiError for $ident {
            fn code() -> i32 {
                ::error::ErrorCode::$ident as i32
            }
        }

        impl ::std::error::Error for $ident {
            fn description(&self) -> &str {
                &self.description
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