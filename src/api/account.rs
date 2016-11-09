use std::collections::HashMap;
use std::sync::RwLock;
use std::sync::Arc;
use std::borrow::Borrow;

use router::Router;
use hyper::header::CookiePair;
use postgres::Connection;
use iron::prelude::*;
use oven::prelude::*;
use params::{Params, Value, FromValue};
use rustc_serialize::json;
use hyper::status::StatusCode; 
use std::io::Read;
use std::str::FromStr;
use std::i32;

use ::api::authorization::Authorizer;
use ::proto::schema::*;
use ::proto::error::*;
use ::proto::response::*;
use ::db::schema::*;
use ::db::builder::*;
use ::db::*;

pub fn get_bookings_handle(req: &mut Request) -> IronResult<Response> {    
    let id = match Authorizer::authorize_request(req) {
        Ok(id) => id,
        Err(err) => return Ok(err.into_api_response().into())
    };

    let ofst = req.get_ref::<Params>().unwrap()
        .find(&["offset"])
        .map(|val| i32::from_value(val).unwrap_or(0))
        .unwrap_or(0);

    let cnt = req.extensions.get::<Router>().unwrap()
        .find("cnt")
        .map(|s| i32::from_str(s).unwrap())
        .unwrap_or(i32::MAX);
    
    let conn = get_db_connection();
    let rows = conn.query(&Booking::select_builder()
                    .filter("ClientPersonID = $1")
                    .limit(cnt)
                    .offset(ofst)
                    .build(),
                    &[&id]).unwrap();
    
    let bookings = rows.into_iter().map(Booking::from).collect::<Vec<Booking>>();
    Ok(ApiResponse::Ok(bookings).into())
}

