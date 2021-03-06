extern crate cookie;
extern crate rustc_serialize;
extern crate hyper;
extern crate chrono;
extern crate rand;
extern crate threadpool;

mod authorize;
mod response;
mod schema;

use rustc_serialize::json;
use response::random_str;
use chrono::NaiveDateTime;

fn main() {
    let (owner_token, _) = authorize::signin_with("owner", "0");
    
    let cities = ["Abakan", "Amsterdam", "Athens", "Antalia", "Rome", "Budapest", "Helsinki", "Oslo", "Stockholm", "Copenhagen", "Moscow", "Kazan", "Innopolis", "Samara", "Saint Petersburg", "Madrid", "Paris", "London", "New York", "Berlin", "Warsaw", "Vienna"];
    let cities_cnt = cities.len() as i32;

    for city in &cities {
        insert_city(city);
    }

    let pool = threadpool::ThreadPool::new(16);

    for i in 0..1000 {
        insert_hotel(&owner_token, schema::NewHotel {
            CityID: (rand::random::<i32>() % cities_cnt).abs() + 1,
            Name: random_str(),
            Description: random_str(),
            Stars: Some((rand::random::<i32>() % 5).abs()),
        });
        
        for r in 0..1000 {
            let owner_token = owner_token.clone();
            pool.execute(move || {
                insert_room(&owner_token, i+1, schema::NewRoom {
                    RoomNumber: r, 
                    RoomLevel: (rand::random::<i32>() % 4).abs(),
                    PhotoSetID: None
                });
            });
        }

        for b in 0..1000 {
            pool.execute(move || {
                let client_token = authorize::signup_with(&random_str(), &random_str(), &random_str(), &random_str());
                insert_booking(&client_token, schema::NewBooking {
                    ClientPersonID: None,
                    HotelID: Some(i+1),
                    RoomNumber: b,
                    ArrivalTime: chrono::UTC::now().naive_local(),
                    DepartureTime: chrono::UTC::now().naive_local(),
                });
            });
        }

	while pool.active_count() != 0 { std::thread::sleep_ms(100); }
    }
}

use authorize::SignedRequest;

fn insert_hotel(token: &str, hotel: schema::NewHotel) {
    let client = hyper::Client::new();
    let mut res = client.put("http://localhost:8080/api/hotel/")
        .body(&json::encode(&hotel).unwrap())
        .sign(token.to_owned())
        .send()
        .unwrap();

    if res.status != hyper::status::StatusCode::Ok {
        std::io::copy(&mut res, &mut std::io::stderr());
        panic!("Got code {:?}", res.status);
    }

}

fn insert_room(token: &str, hotel_id: i32, room: schema::NewRoom) {
    let client = hyper::Client::new();
    let mut res = client.put(&format!("http://localhost:8080/api/hotel/{}/room/", hotel_id))
        .body(&json::encode(&room).unwrap())
        .sign(token.to_owned())
        .send()
        .unwrap();

    if res.status != hyper::status::StatusCode::Ok {
        std::io::copy(&mut res, &mut std::io::stderr());
        panic!("Got code {:?}", res.status);
    }
}

fn insert_booking(token: &str, booking: schema::NewBooking) {
    let client = hyper::Client::new();
    let mut res = client.put("http://localhost:8080/api/booking/")
        .body(&json::encode(&booking).unwrap())
        .sign(token.to_owned())
        .send()
        .unwrap();

    if res.status != hyper::status::StatusCode::Ok {
        std::io::copy(&mut res, &mut std::io::stderr());
        panic!("Got code {:?}", res.status);
    }
}

fn insert_city(name: &str) {
    let client = hyper::Client::new();
    let mut res = client.put("http://localhost:8080/api/city/")
          .body(&format!("{{\"Name\":\"{}\"}}", name))
          .send()
          .unwrap();

    if res.status != hyper::status::StatusCode::Ok {
        std::io::copy(&mut res, &mut std::io::stderr());
        panic!("Got code {:?}", res.status);
    }
}