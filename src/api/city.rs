use iron::prelude::*;
use rustc_serialize::json;
use hyper::status::StatusCode;

use super::request_body;
use ::proto::response::*;
use ::proto::error::*;
use ::proto::schema::NewCity;
use ::db::schema::*;
use ::db::*;

pub fn get_cities(_: &mut Request) -> IronResult<Response> {
    let query = City::select_builder().build();
    let conn = get_db_connection();

    info!("request GET /city");

    let rows = conn.query(&query, &[]).unwrap();
    let cities = rows.into_iter()
        .map(City::from)
        .collect::<Vec<City>>();

    Ok(ApiResponse::Ok(cities).into())
}

pub fn put_city(req: &mut Request) -> IronResult<Response> {
    let new_city: NewCity = match json::decode(&request_body(req)) {
        Ok(city) => city,
        Err(err) => return Ok(InvalidSchemaError::from(err).into_api_response().into()),
    };

    info!("request PUT /city {{ {:?} }}", new_city);

    let conn = get_db_connection();
    let query = City::insert_query();

    conn.execute(&query, &[&new_city.Name]).unwrap();

    Ok(Response::with(StatusCode::Ok))
}