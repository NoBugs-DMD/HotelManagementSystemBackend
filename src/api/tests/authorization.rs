use hyper;
use hyper::client::RequestBuilder;
use hyper::header::{Cookie, SetCookie, CookiePair};
use hyper::status::StatusCode;
use rustc_serialize::json;
use rand;

use super::random_str;
use super::response_body;
use ::proto::schema::*;
use ::db::schema::*;
use ::db::*;

#[test]
pub fn signup() {
    signup_random();    
}

#[test]
fn signin() {
    let login = format!("{}", random_str());
    let name =  format!("{}", random_str());
    let email = format!("{}", random_str());
    let passhash = format!("{}", random_str());

    get_db_connection().execute(&Person::insert_query(), &Person {
        ID:       0,
        Login:    login.clone(),
        Name:     name,
        Email:    email,
        PassHash: passhash.clone(),
    }.insert_args())
     .unwrap();

    signin_with(&login, &passhash);
}

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
    assert_eq!(roles, Roles {
        Client: true, 
        Owner: false, 
        Manager: false, 
        Cleaner: false, 
        Receptionist: false
    });

    token.value.to_owned()
}

pub fn signup_random() -> String {
    signup_with(&random_str(), &random_str(), &random_str(), &random_str())    
}
