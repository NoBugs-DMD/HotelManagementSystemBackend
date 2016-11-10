use hyper;
use hyper::header::SetCookie;
use hyper::status::StatusCode;
use rustc_serialize::json;
use rand;

use super::response_body;
use ::proto::schema::*;
use ::db::schema::*;
use ::db::*;

fn random() -> u32 {
    use rand::Rng;
    rand::thread_rng().next_u32()
}

#[test]
pub fn signup() {
    signup_random();    
}

#[test]
fn signin() {
    let login = format!("{}", random());
    let name =  format!("{}", random());
    let email = format!("{}", random());
    let passhash = format!("{}", random());

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

pub fn signup_random() -> String {
    let client = hyper::Client::new();
    let mut res = client.post("http://localhost:8080/api/signup/")
        .body(&format!("{{ \"Login\":\"{}\", \"Name\":\"{}\", \"Email\":\"{}\", \
                 \"PassHash\":\"{}\"}}", random(), random(), random(), random()))
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
