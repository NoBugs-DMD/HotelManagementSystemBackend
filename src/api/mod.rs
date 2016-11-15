pub mod authorization;
pub mod city;
pub mod account;
pub mod booking;
pub mod hotel;
pub mod ruleset;
pub mod manager;

#[cfg(test)]
mod tests;

use iron::prelude::*;
use std::io::Read;
use rustc_serialize::json;
use rustc_serialize::Decodable;
use ::proto::error::*;

fn request_body<T: Decodable>(req: &mut Request) -> ApiResult<T> {
    let mut buffer = String::with_capacity(128);
    req.body.read_to_string(&mut buffer).unwrap();
    debug!("request body: {}", buffer);
        
    decode_json(&buffer)
}

fn decode_json<T: Decodable>(json: &str) -> ApiResult<T> {
    json::decode(json).map_err(|err| box InvalidSchemaError::from(err) as Box<ApiError>)
}