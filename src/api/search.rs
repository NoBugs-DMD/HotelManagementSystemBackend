use iron::prelude::*;
use router::Router;
use hyper::status::StatusCode;
use postgres::types::ToSql;
use std::str::FromStr;
use std::i32;
use rustc_serialize::json;
use postgres::Connection;
use postgres::error::{Error, DbError, SqlState};
use chrono::NaiveDateTime;

use super::request_body;
use super::decode_json;
use ::api::authorization::Authorizer;
use ::proto::response::*;
use ::proto::error::*;
use ::proto::schema::*;
use ::db::schema::*;
use ::db::*;

pub fn search(req: &mut Request) -> IronResult<Response> {
    let conn = get_db_connection();
    let user = Authorizer::authorize_request(&conn, req)?;
    let search_req: SearchRequest = request_body(req)?;

    let mut where_clause = if let Some(hotel_id) = search_req.HotelID {
        format!("Hotel.ID = {} and Room.HotelID = Hotel.ID", hotel_id)
    } else {
        format!("Hotel.CityID = {} and Room.HotelID = Hotel.ID",
                search_req.CityID)
    };

    rating_range_to_clause(search_req.Rating).map(|clause| where_clause.push_str(&clause));
    stars_range_to_clause(search_req.Stars).map(|clause| where_clause.push_str(&clause));
    price_range_to_clause(user.id, search_req.Price).map(|clause| where_clause.push_str(&clause));
    date_range_to_clause(search_req.DateTime).map(|clause| where_clause.push_str(&clause));

    let rooms = conn.query(&SelectQueryBuilder::default()
                   .columns("Room.*")
                   .from_tables("Room, Hotel")
                   .filter(where_clause)
                   .group_by("Room.HotelID, Room.RoomLevel")
                   .build(),
               &[])
        .unwrap()
        .into_iter()
        .map(Room::from)
        .map(|room| PricedRoom::from_room(&conn, room, user.id))
        .collect::<Vec<PricedRoom>>();

    Ok(rooms.as_response())
}

fn rating_range_to_clause(rating: Option<Range<i32>>) -> Option<String> {
    rating.map(|r| {
        format!(" and (Hotel.Rating >= {} and Hotel.Rating <= {}) or Hote.Rating = null",
                r.from,
                r.to)
    })
}

fn date_range_to_clause(datetime: Option<Range<NaiveDateTime>>) -> Option<String> {
    datetime.map(|dt| {
        format!(" and NOT EXISTS( SELECT * FROM Booking WHERE Booking.HotelID = Room.HotelID \
                  and Booking.RoomNumber = Room.RoomNumber \
                  and (Booking.ArrivalTime > {} or Booking.DepartureTime < {}); )",
                dt.from,
                dt.to)
    })
}

fn stars_range_to_clause(stars: Option<Range<i32>>) -> Option<String> {
    stars.map(|s| {
        format!(" and (Hotel.Stars >= {} and Hotel.Stars <= {})",
                s.from,
                s.to)
    })
}

fn price_range_to_clause(client_id: i32, price: Option<Range<i32>>) -> Option<String> {
    price.map(|p| {
        format!(" and (SELECT room_price(Room.RoomLevel, Room.HotelID, {})) >= {} and (SELECT \
                 room_price(Room.RoomLevel, Room.HotelID, {})) <= {}",
                client_id,
                p.from,
                client_id,
                p.to)
    })
}
