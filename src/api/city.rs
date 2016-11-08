use hyper::header::CookiePair;
use postgres::Connection;
use iron::prelude::*;
use oven::prelude::*;
use rustc_serialize::json;
use rustc_serialize::Encodable;
use hyper::status::StatusCode; 
use std::io::Read;

use ::proto::schema::*;
use ::proto::response::*;
use ::proto::error::*;
use ::db::schema::*;
use ::db::builder::*;
use ::db::*;

pub fn get_cities_handler(req: &mut Request) -> IronResult<Response> {
    let query = City::select_builder().build();
    let conn = get_db_connection();

    let rows = conn.query(&query, &[]).unwrap();
    let cities = rows.into_iter().map(City::from)
                                 .collect::<Vec<City>>();

    ApiResponse::Ok(cities).into()
}

pub fn put_city_handler(req: &mut Request) -> IronResult<Response> {
    let mut buffer = String::with_capacity(32);
    req.body.read_to_string(&mut buffer).unwrap();

    let new_city: City = match json::decode(&buffer) {
        Ok(city) => city,
        Err(err) => return InvalidSchemaError::from(err).into_api_response().into()
    };

    let conn = get_db_connection();
    let query = City::insert_query();

    conn.execute(&query, &[&new_city.Name]).unwrap();

    Ok(Response::with(StatusCode::Ok))    
}