use hyper;
use hyper::client::RequestBuilder;
use hyper::header::{Cookie, SetCookie, CookiePair};
use hyper::status::StatusCode;
use rustc_serialize::json;

use ::response::random_str;
use ::response::response_body;
use ::schema::*;

pub trait SignedRequest {
    fn sign(self, token: String) -> Self;
} 

impl<'a> SignedRequest for RequestBuilder<'a> {
    fn sign(self, token: String) -> RequestBuilder<'a> {
        self.header(
            Cookie(vec![
                CookiePair::new("token".to_owned(), token),
            ])
        )
    }
}

pub fn signin_with(login: &str, passwd: &str) -> (String, Roles) {
    let client = hyper::Client::new();
    let mut res = client.post("http://localhost:8080/api/signin/")
        .body(&format!("{{ \"Login\":\"{}\", \"PassHash\":\"{}\"}}", login, passwd))
        .send()
        .unwrap();
    
    let resp_body = response_body(&mut res);

    let token = res.headers.get::<SetCookie>().unwrap()
        .iter()
        .filter(|cp| cp.name == "token")
        .last()
        .unwrap();

    let roles: Roles = json::decode(&resp_body).unwrap();
    assert_eq!(res.status, StatusCode::Ok);

    (token.value.to_owned(), roles)
}

pub fn signup_with(login: &str, name: &str, email: &str, passhash: &str) -> String {
    let client = hyper::Client::new();
    let mut res = client.post("http://localhost:8080/api/signup/")
        .body(&format!("{{ \"Login\":\"{}\", \"Name\":\"{}\", \"Email\":\"{}\", \
                 \"PassHash\":\"{}\"}}", login, name, email, passhash))
        .send()
        .unwrap();
    
    let resp_body = response_body(&mut res);
    println!("body:    {:?}", resp_body);
    println!("headers: {:?}", res.headers);

    let token = res.headers.get::<SetCookie>().unwrap()
        .iter()
        .filter(|cp| cp.name == "token")
        .last()
        .unwrap();

    println!("token: {:?}", token.value);

    let roles: Roles = json::decode(&resp_body).unwrap();
    assert_eq!(res.status, StatusCode::Ok);

    token.value.to_owned()
}