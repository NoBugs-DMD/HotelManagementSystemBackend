use rustc_serialize::Encodable;
use rustc_serialize::json;
use iron::prelude::*;
use hyper::status::StatusCode;

pub trait AsApiResponse {
    fn as_response(&self) -> Response;
}

impl<D: Sized + Encodable> AsApiResponse for D {
    fn as_response(&self) -> Response {
        let mut response = Response::with(StatusCode::Ok);
        response.body = Some(box json::encode(self).unwrap());
        response
    }
}  