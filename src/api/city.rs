use iron::prelude::*;
use rustc_serialize::json;
use hyper::status::StatusCode;
use std::io::Read;

use ::proto::schema::*;
use ::proto::response::*;
use ::proto::error::*;
use ::db::schema::*;
use ::db::*;

pub fn get_cities_handler(_: &mut Request) -> IronResult<Response> {
    let query = City::select_builder().build();
    let conn = get_db_connection();

    info!("request GET /city");

    let rows = conn.query(&query, &[]).unwrap();
    let cities = rows.into_iter()
        .map(City::from)
        .collect::<Vec<City>>();

    Ok(ApiResponse::Ok(cities).into())
}

pub fn put_city_handler(req: &mut Request) -> IronResult<Response> {
    let mut buffer = String::with_capacity(32);
    req.body.read_to_string(&mut buffer).unwrap();

    let new_city: City = match json::decode(&buffer) {
        Ok(city) => city,
        Err(err) => return Ok(InvalidSchemaError::from(err).into_api_response().into()),
    };

    info!("request POST /city {{ {:?} }}", new_city);

    let conn = get_db_connection();
    let query = City::insert_query();

    conn.execute(&query, &[&new_city.Name]).unwrap();

    Ok(Response::with(StatusCode::Ok))
}