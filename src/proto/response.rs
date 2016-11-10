use rustc_serialize::Encodable;
use rustc_serialize::json::{self, EncodeResult};
use iron::prelude::*;
use hyper::status::StatusCode;
use std::borrow::Cow;

#[derive(Debug, Clone, RustcEncodable)]
pub enum ApiResponse<D>
    where D: Encodable
{
    Ok(D),
    Err(i32, Cow<'static, str>),
}

impl<D> ApiResponse<D>
    where D: Encodable
{
    pub fn to_json(&self) -> EncodeResult<String> {
        match *self {
            ApiResponse::Ok(ref data) => json::encode(&data),
            ApiResponse::Err(ref code, ref desc) => {
                Ok(format!("{{ \"error_code\": {}, \"description\": \"{}\" }}",
                           code,
                           &desc))
            }
        }
    }
}

impl<D> Into<Response> for ApiResponse<D>
    where D: Encodable
{
    fn into(self) -> Response {
        let mut response = match self { 
            ApiResponse::Ok(_) => Response::with(StatusCode::Ok),
            ApiResponse::Err(..) => Response::with(StatusCode::Forbidden),
        };

        response.body = Some(box self.to_json().unwrap());

        response
    }
}