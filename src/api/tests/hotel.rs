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
fn get_hotels() {
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

    let hotel = hotels.into_iter()
        .filter(|hotel|
            hotel.CityID == 3 &&
            hotel.Name == name &&
            hotel.Description == desc &&
            hotel.Stars == Some(5))
        .last()
        .expect("No putted hotel in get result");

    let client = hyper::Client::new();
    let mut res = client.get(&format!("http://localhost:8080/api/hotel/{}", hotel.ID))
        .send()
        .unwrap();

    let resp_body = response_body(&mut res);
    println!("body:    {:?}", resp_body);
    println!("headers: {:?}", res.headers);    
    assert_eq!(res.status, StatusCode::Ok);
    
    let recv_hotel: Hotel = json::decode(&resp_body).unwrap();

    assert_eq!(hotel, recv_hotel);
}

#[test]
fn update_hotel() {
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
        .sign(token.clone())
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

    let hotel = hotels.into_iter()
        .filter(|hotel|
            hotel.CityID == 3 &&
            hotel.Name == name &&
            hotel.Description == desc &&
            hotel.Stars == Some(5))
        .last()
        .expect("No putted hotel in get result");
    
    let client = hyper::Client::new();
    let mut res = client.post(&format!("http://localhost:8080/api/hotel/{}", hotel.ID))
        .body(&json::encode(&UpdateHotel {
            RuleSetID: Some(5),
            Name: None,
            Description: None,
            PhotoSetID: Some(5),
            Stars: Some(3)
        }).unwrap())
        .sign(token)
        .send()
        .unwrap();
    
    let resp_body = response_body(&mut res);
    println!("body:    {:?}", resp_body);
    println!("headers: {:?}", res.headers);    
    assert_eq!(res.status, StatusCode::Ok);
    
    let mut res = client.get(&format!("http://localhost:8080/api/hotel/{}", hotel.ID))
        .send()
        .unwrap();

    let resp_body = response_body(&mut res);
    println!("body:    {:?}", resp_body);
    println!("headers: {:?}", res.headers);    
    assert_eq!(res.status, StatusCode::Ok);
    
    let hotel: Hotel = json::decode(&resp_body).unwrap();

    assert_eq!(hotel.RuleSetID, 5);
    assert_eq!(hotel.PhotoSetID, Some(5));
    assert_eq!(hotel.Stars, Some(3));
}

// TODO Test all of hotels api