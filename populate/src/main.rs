extern crate cookie;
extern crate rustc_serialize;
extern crate hyper;
extern crate chrono;
extern crate rand;

mod authorize;
mod response;
mod schema;

fn main() {
    let (owner_token, _) = authorize::signin_with("owner", "0");
    
    for city in &["Abakan", "Amsterdam", "Athens", "Antalia", "Rome", "Budapest", "Helsinki", "Oslo", "Stockholm", "Copenhagen", "Moscow", "Kazan", "Innopolis", "Samara", "Saint Petersburg", "Madrid", "Paris", "London", "New York", "Berlin", "Warsaw", "Vienna"] {
        insert_city(city);
    }
}

fn insert_hotel(token: &str, hotel: schema::NewHotel) {
    unimplemented!()
}

fn insert_city(name: &str) {
    let client = hyper::Client::new();
    client.put("http://localhost:8080/api/city/")
          .body(&format!("{{\"Name\":\"{}\"}}", name))
          .send()
          .unwrap();
}