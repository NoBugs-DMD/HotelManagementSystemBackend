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
fn put_hotel() {
    let token = signin_owner();

    let name = random_str();
    let desc = random_str();
    
    let client = hyper::Client::new();
    let mut res = client.put("http://localhost:8080/api/hotel/")
        .body(&json::encode(&NewHotel {
            CityID: 3,
            Name: name,
            Description: desc,
            Stars: Some(5)
        }).unwrap())
        .sign(token)
        .send()
        .unwrap();


    let resp_body = response_body(&mut res);
    println!("body:    {:?}", resp_body);
    println!("headers: {:?}", res.headers);
    assert_eq!(res.status, StatusCode::Ok);
}

#[test]
fn get_hotel() {
    let name = random_str();
    let desc = random_str();

    let token = signin_owner();
    let client = hyper::Client::new();
    let mut res = client.put("http://localhost:8080/api/hotel/")
        .body(&json::encode(&NewHotel {
            CityID: 3,
            Name: name.clone(),
            Description: desc.clone(),
            Stars: Some(5)
        }).unwrap())
        .sign(token)
        .send()
        .unwrap();


    let resp_body = response_body(&mut res);
    println!("body:    {:?}", resp_body);
    println!("headers: {:?}", res.headers);
    assert_eq!(res.status, StatusCode::Ok);

    let client = hyper::Client::new();
    let mut res = client.get("http://localhost:8080/api/hotels/").send().unwrap();
    
    let resp_body = response_body(&mut res);
    println!("body:    {:?}", resp_body);
    println!("headers: {:?}", res.headers);    
    assert_eq!(res.status, StatusCode::Ok);
    
    
    let hotels: Vec<Hotel> = json::decode(&resp_body).unwrap();

    hotels.into_iter()
        .filter(|hotel|
            hotel.CityID == 3 &&
            hotel.Name == name &&
            hotel.Description == desc &&
            hotel.Stars == Some(5))
        .last()
        .expect("No putted hotel in get result");
}