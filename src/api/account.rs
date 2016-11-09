use router::Router;
use iron::prelude::*;
use params::{Params, FromValue};
use std::str::FromStr;
use std::i32;

use ::api::authorization::Authorizer;
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
 
    info!("request GET /account/bookings {{ id: {}, cnt: {}, ofst: {} }}", id, cnt, ofst);
    
    let conn = get_db_connection();
    let rows = conn.query(&Booking::select_builder()
        .filter("ClientPersonID = $1")
        .limit(cnt)
        .offset(ofst)
        .build(),
        &[&id]
    ).unwrap();
    
    let bookings = rows.into_iter().map(Booking::from).collect::<Vec<Booking>>();
    Ok(ApiResponse::Ok(bookings).into())
}

