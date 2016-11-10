mod account;
mod authorization;
mod booking;
mod city;

use hyper::client::response::Response;
use std::io::Read;

fn response_body(res: &mut Response) -> String {
    let mut buffer = String::with_capacity(128);
    res.read_to_string(&mut buffer).unwrap();

    debug!("response body: {}", buffer);

    buffer
}
