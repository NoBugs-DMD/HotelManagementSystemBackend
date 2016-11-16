use hyper::client::response::Response;
use std::io::Read;
use rand;

pub fn response_body(res: &mut Response) -> String {
    let mut buffer = String::with_capacity(128);
    res.read_to_string(&mut buffer).unwrap();

    buffer
}

pub fn random_str() -> String {
    use rand::AsciiGenerator;
    use rand::Rng;

    rand::thread_rng()
        .gen_ascii_chars()
        .take(10)
        .collect()
}

