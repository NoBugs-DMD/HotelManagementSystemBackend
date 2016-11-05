use rustc_serialize::{Encodable};
use rustc_serialize::json::{self, EncodeResult};
use iron::prelude::*;
use hyper::status::StatusCode; 

#[derive(Debug, Clone, RustcEncodable)]
pub enum ApiResponse<D> 
    where D: Encodable
{
    Ok(D),
    Err(i32, String)
}

impl<D> ApiResponse<D> 
    where D: Encodable
{
    pub fn to_json(&self) -> EncodeResult<String> {
        match *self {
            ApiResponse::Ok(ref data) => json::encode(&data),
            ApiResponse::Err(ref code, ref desc) => Ok( 
                format!("{{ \"error_code\": {}, \"description\": \"{}\" }}", code, desc)
            )
        }        
    }
}

impl<D> Into<IronResult<Response>> for ApiResponse<D> 
    where D: Encodable
{
    fn into(self) -> IronResult<Response> {
        let mut response = match self { 
            ApiResponse::Ok(_) => Response::with(StatusCode::Ok),
            ApiResponse::Err(..) => Response::with(StatusCode::Forbidden),
        };

        response.body = Some(box self.to_json().unwrap());

        Ok(response)
    }
}

#[macro_export]
macro_rules! api_error_into_api_response {
    ($ty:ty) => {
        impl Into<::response::ApiResponse<()>> for $ty {
            fn into(self) -> ::response::ApiResponse<()> {
                ::response::ApiResponse::Err (
                    <Self as ::error::ApiError>::code(),
                    self.description,
                )
            }
        }
    }
}

#[macro_export]
macro_rules! data_into_api_response {
    ($ty:ty) => {
        impl Into<::response::ApiResponse<$ty>> for $ty {
            fn into(self) -> ::response::ApiResponse<$ty> {
                ::response::ApiResponse::Ok {
                    data: self
                }
            }
        }
    }
}