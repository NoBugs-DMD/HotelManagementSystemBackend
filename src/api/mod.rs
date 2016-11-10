pub mod authorization;
pub mod city;
pub mod account;
pub mod booking;

use iron::prelude::*;
use std::io::Read;

fn request_body(req: &mut Request) -> String {
    let mut buffer = String::with_capacity(128);
    req.body.read_to_string(&mut buffer).unwrap();

    debug!("request body: {}", buffer);

    buffer
}