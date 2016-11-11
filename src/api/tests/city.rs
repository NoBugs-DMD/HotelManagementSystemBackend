use hyper;
use hyper::status::StatusCode;
use rustc_serialize::json;

use super::response_body;
use super::random_str;
use ::db::schema::*;

#[test]
fn city() {
    let client = hyper::Client::new();
    let city_name = random_str();
    let mut res = client.put("http://localhost:8080/api/city/")
        .body(&format!("{{ \"Name\":\"{}\" }}", city_name))
        .send()
        .unwrap();

    let resp_body = response_body(&mut res);
    println!("body:    {:?}", resp_body);
    println!("headers: {:?}", res.headers);

    assert_eq!(resp_body, "");
    assert_eq!(res.status, StatusCode::Ok);

    let mut res = client.get("http://localhost:8080/api/city/")
        .send()
        .unwrap();

    let resp_body = response_body(&mut res);
    println!("body:    {:?}", resp_body);
    println!("headers: {:?}", res.headers);

    let cities: Vec<City> = json::decode(&resp_body).unwrap();

    assert_eq!(res.status, StatusCode::Ok);
    assert_eq!(cities.into_iter().filter(|city| city.Name == city_name).count(), 1);
}