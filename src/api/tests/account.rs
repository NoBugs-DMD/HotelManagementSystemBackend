use hyper;
use hyper::client::request::Request;
use hyper::header::{SetCookie, CookiePair};
use hyper::status::StatusCode;
use rustc_serialize::json;
use rand;

use super::random_str;
use super::response_body;
use super::authorization::*;
use ::proto::schema::*;
use ::db::schemaext::*;
use ::db::schema::*;
use ::db::*;

#[test]
fn get_account_info() {
    let login = random_str();
    let name = random_str();
    let email = random_str();
    let passhash = random_str();

    let token = signup_with(&login, &name, &email, &passhash);
    let info = account_info(token);

    assert_eq!(login, info.Login);
    assert_eq!(name, info.Name);
    assert_eq!(email, info.Email);
}

fn account_info(token: String) -> AccountInfo {
    let client = hyper::Client::new();
    let mut res = client.get("http://localhost:8080/api/account/")
        .sign(token)
        .send()
        .unwrap();

    let resp_body = response_body(&mut res);
    println!("body:    {:?}", resp_body);
    println!("headers: {:?}", res.headers);

    assert_eq!(res.status, StatusCode::Ok);

    json::decode(&resp_body).unwrap()
}

#[test]
fn update_account_info() {
    let login = random_str();
    let name = random_str();
    let email = random_str();
    let passhash = random_str();

    let token = signup_with(&login, &name, &email, &passhash);
    
    let new_name = random_str();
    let new_email = random_str();
    let new_passhash = random_str();

    let client = hyper::Client::new();

    // Test empty request
    let mut res = client.post("http://localhost:8080/api/account/")
        .sign(token.clone())
        .body("{}")
        .send()
        .unwrap();

    assert_eq!(res.status, StatusCode::Ok);

    // Test updating just name and email 
    let update = UpdateAccountInfoData {
        NewName: Some(new_name.clone()),
        NewEmail: Some(new_email.clone()),
        OldPassHash: None,
        NewPassHash: None
    };

    let mut res = client.post("http://localhost:8080/api/account/")
        .sign(token.clone())
        .body(&json::encode(&update).unwrap())
        .send()
        .unwrap();

    assert_eq!(res.status, StatusCode::Ok);
    
    let info = account_info(token.clone());
    assert_eq!(info.Name, new_name);
    assert_eq!(info.Email, new_email);

    // Try to update password without old passwoord
    let update = UpdateAccountInfoData {
        NewName: None,
        NewEmail: None,
        OldPassHash: None,
        NewPassHash: Some(new_passhash.clone())
    };

    let mut res = client.post("http://localhost:8080/api/account/")
        .sign(token.clone())
        .body(&json::encode(&update).unwrap())
        .send()
        .unwrap();

    assert_eq!(res.status, StatusCode::Forbidden);

    // Update password
    let update = UpdateAccountInfoData {
        NewName: None,
        NewEmail: None,
        OldPassHash: Some(passhash.clone()),
        NewPassHash: Some(new_passhash.clone())
    };

    let mut res = client.post("http://localhost:8080/api/account/")
        .sign(token.clone())
        .body(&json::encode(&update).unwrap())
        .send()
        .unwrap();

    assert_eq!(res.status, StatusCode::Ok);

    // Try to authorize with new password
    signin_with(&login, &new_passhash);
}

#[test]
fn get_bookings() {
    let login = random_str();
    let name = random_str();
    let email = random_str();
    let passhash = random_str();

    let token = signup_with(&login, &name, &email, &passhash);

    // TODO implement hotels API and use it in this test
}