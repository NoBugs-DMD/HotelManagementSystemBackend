mod account;
mod authorization;
mod booking;
mod city;

use hyper::client::response::Response;
use std::io::Read;
use rand;

fn response_body(res: &mut Response) -> String {
    let mut buffer = String::with_capacity(128);
    res.read_to_string(&mut buffer).unwrap();

    debug!("response body: {}", buffer);

    buffer
}

fn random_str() -> String {
    use rand::AsciiGenerator;
    use rand::Rng;

    rand::thread_rng()
        .gen_ascii_chars()
        .take(10)
        .collect()
}

