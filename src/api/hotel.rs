use iron::prelude::*;
use router::Router;
use rustc_serialize::json;
use hyper::status::StatusCode;
use params::{Params, FromValue};
use postgres::types::ToSql;
use std::str::FromStr;
use std::i32;

use super::request_body;
use ::api::authorization::Authorizer;
use ::api::ruleset::DEFAULT_RULESET_ID;
use ::proto::response::*;
use ::proto::error::*;
use ::proto::schema::*;
use ::db::schema::*;
use ::db::*;

pub fn get_hotels(req: &mut Request) -> IronResult<Response> {
    let ofst = req.get_ref::<Params>()
        .unwrap()
        .find(&["offset"])
        .map(|val| i32::from_value(val).unwrap_or(0))
        .unwrap_or(0);

    let cnt = req.extensions
        .get::<Router>()
        .unwrap()
        .find("cnt")
        .map(|s| i32::from_str(s).unwrap())
        .unwrap_or(i32::MAX);

    let conn = get_db_connection();
    let hotels = conn.query(&Hotel::select_builder()
                   .limit(cnt)
                   .offset(ofst)
                   .build(),
               &[])
        .unwrap()
        .into_iter()
        .map(Hotel::from)
        .collect::<Vec<Hotel>>();

    Ok(ApiResponse::Ok(hotels).into())
}

pub fn get_hotel(req: &mut Request) -> IronResult<Response> {
    let hotel_id = req.extensions
        .get::<Router>()
        .unwrap()
        .find("id")
        .map(|s| i32::from_str(s).unwrap())
        .expect("No Hotel ID in request");
    
    let conn = get_db_connection();
    let hotel = conn.query(&Hotel::select_builder()
                   .filter("ID = $1")
                   .build(),
               &[&hotel_id])
        .unwrap()
        .into_iter()
        .last()
        .map(Hotel::from);

    match hotel {
        Some(hotel) => Ok(ApiResponse::Ok(hotel).into()),
        None => { 
            Ok(NotFoundError::from_str(format!("No Hotel with id {} found", hotel_id))
                .into_api_response()
                .into())
        }
    }
}

pub fn put_hotel(req: &mut Request) -> IronResult<Response> {
    let conn = get_db_connection();
    let client = authorize!(conn, req);

    if !client.roles.Owner {
        return Ok(NotAuthorizedError::from_str("Only owner can add a hotel")
            .into_api_response()
            .into());
    }

    let new_hotel: NewHotel = decode_body!(req);
    let hotel = Hotel::new(
        client.id,
        DEFAULT_RULESET_ID,
        new_hotel.CityID,
        None,
        new_hotel.Name,
        new_hotel.Description,
        None,
        new_hotel.Stars
    );

    conn.execute(&Hotel::insert_query(), &hotel.insert_args())
        .unwrap();

    Ok(Response::with(StatusCode::Ok))
}

pub fn update_hotel(req: &mut Request) -> IronResult<Response> {
    let hotel_id = req.extensions
        .get::<Router>()
        .unwrap()
        .find("id")
        .map(|s| i32::from_str(s).unwrap())
        .expect("No Hotel ID in request");

    let conn = get_db_connection();
    let client = authorize!(conn, req);

    if !client.roles.Owner && !client.roles.Manager {
        return Ok(NotAuthorizedError::from_str("Only owner and manager update hotel's info")
            .into_api_response()
            .into());
    }

    let update_hotel: UpdateHotel = decode_body!(req);

    // Check authority in the hotel-to-update
    if !client.roles.Owns.map_or(false, |owns| owns.contains(&hotel_id)) 
    && !client.roles.EmployedIn.map_or(false, |emp| emp.contains(&hotel_id)) {
        return Ok(NotAuthorizedError::from_str(format!("Not owner or employee of hotel {}", hotel_id))
            .into_api_response()
            .into());
    }

    let mut update = Hotel::update_builder().filter(format!("HotelID = {}", hotel_id));
    let mut values: Vec<&ToSql> = Vec::with_capacity(4);

    if let Some(id) = update_hotel.RuleSetID.as_ref() {
        update = update.set("RuleSetID");
        values.push(id);
    }

    if let Some(name) = update_hotel.Name.as_ref() {
        update = update.set("Name");
        values.push(name);
    }

    if let Some(desc) = update_hotel.Description.as_ref() {
        update = update.set("Description");
        values.push(desc);
    }

    if let Some(photoset) = update_hotel.PhotoSetID.as_ref() {
        update = update.set("PhotoSetID");
        values.push(photoset);
    }

    if let Some(stars) = update_hotel.Stars.as_ref() {
        update = update.set("Stars");
        values.push(stars);
    }

    // Early exit if we got empty json
    if values.is_empty() {
        return Ok(Response::with(StatusCode::Ok));
    }

    conn.execute(&update.build(), &values)
    .unwrap();

    Ok(Response::with(StatusCode::Ok))
}

pub fn get_rooms(req: &mut Request) -> IronResult<Response> {
    unimplemented!()
}

pub fn get_room(req: &mut Request) -> IronResult<Response> {
    unimplemented!()
}

pub fn put_room(req: &mut Request) -> IronResult<Response> {
    unimplemented!()
}

pub fn update_room(req: &mut Request) -> IronResult<Response> {
    unimplemented!()
}

pub fn get_reviews(req: &mut Request) -> IronResult<Response> {
    unimplemented!()
}

pub fn get_employees(req: &mut Request) -> IronResult<Response> {
    unimplemented!()
}

pub fn fire_employee(req: &mut Request) -> IronResult<Response> {
    unimplemented!()
}

pub fn get_ruleset(req: &mut Request) -> IronResult<Response> {
    unimplemented!()
}

pub fn update_ruleset(req: &mut Request) -> IronResult<Response> {
    unimplemented!()
}