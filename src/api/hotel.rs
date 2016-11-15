use iron::prelude::*;
use router::Router;
use hyper::status::StatusCode;
use params::{Params, FromValue};
use postgres::types::ToSql;
use std::str::FromStr;
use std::i32;

use super::request_body;
use ::api::authorization::Authorizer;
use ::api::ruleset;
use ::proto::response::*;
use ::proto::error::*;
use ::proto::schema::*;
use ::db::schema::*;
use ::db::schemaext::*;
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

    info!("request GET /hotels/{}?offset={}", cnt, ofst);

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

    Ok(hotels.as_response())
}

pub fn get_hotel(req: &mut Request) -> IronResult<Response> {
    let hotel_id = req.extensions
        .get::<Router>()
        .unwrap()
        .find("id")
        .map(|s| i32::from_str(s).unwrap())
        .expect("No Hotel ID in request");

    info!("request GET /hotel/{}", hotel_id);

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
        Some(hotel) => Ok(hotel.as_response()),
        None => Err(NotFoundError::from_str(format!("No Hotel with id {} found", hotel_id)).into()),
    }
}

pub fn put_hotel(req: &mut Request) -> IronResult<Response> {
    let conn = get_db_connection();
    let user = Authorizer::authorize_request(&conn, req)?;

    if !user.roles.Owner {
        return Err(NotAuthorizedError::from_str("Only owner can add a hotel").into());
    }

    let new_hotel: NewHotel = request_body(req)?;

    info!("request PUT /hotel/ {{ {:?} }}", new_hotel);

    let hotel = Hotel {
        ID: 0,
        OwnerPersonID: user.id,
        RuleSetID: *ruleset::DEFAULT_RULESET_ID,
        CityID: new_hotel.CityID,
        PhotoSetID: None,
        Name: new_hotel.Name,
        Description: new_hotel.Description,
        Rating: None,
        Stars: new_hotel.Stars
    };

    let hotel_id = procedure::insert_hotel(&conn, hotel);
    ruleset::process_rules(&conn, hotel_id);

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
    let user = Authorizer::authorize_request(&conn, req)?;

    if !user.roles.Owner && !user.roles.Manager {
        return Err(NotAuthorizedError::from_str("Only owner and manager update hotel's info")
            .into());
    }

    let update_hotel: UpdateHotel = request_body(req)?;

    info!("request POST /hotel/ {{ {:?} }}", update_hotel);

    // Check authority in the hotel-to-update
    if !user.roles.Owns.map_or(false, |owns| owns.contains(&hotel_id)) &&
       !user.roles.EmployedIn.map_or(false, |emp| emp.contains(&hotel_id)) {
        return Err(NotAuthorizedError::from_str(format!("Not owner or employee of hotel {}",
                                                        hotel_id))
            .into());
    }

    let mut update = Hotel::update_builder().filter(format!("ID = {}", hotel_id));
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

    if update_hotel.RuleSetID.is_some() {
        ruleset::process_rules(&conn, hotel_id)?;
    }

    Ok(Response::with(StatusCode::Ok))
}

pub fn get_rooms(req: &mut Request) -> IronResult<Response> {
    let ofst = req.get_ref::<Params>()
        .unwrap()
        .find(&["offset"])
        .map(|val| i32::from_value(val).unwrap_or(0))
        .unwrap_or(0);

    let hotel_id = req.extensions
        .get::<Router>()
        .unwrap()
        .find("id")
        .map(|s| i32::from_str(s).unwrap())
        .expect("No Hotel ID in request");

    let cnt = req.extensions
        .get::<Router>()
        .unwrap()
        .find("cnt")
        .map(|s| i32::from_str(s).unwrap())
        .unwrap_or(i32::MAX);


    info!("request GET /hotel/{}/rooms/{}?offset={}",
          hotel_id,
          cnt,
          ofst);

    let conn = get_db_connection();
    let rooms = conn.query(&Room::select_builder()
                   .filter("HotelID = $1")
                   .limit(cnt)
                   .offset(ofst)
                   .build(),
               &[&hotel_id])
        .unwrap()
        .into_iter()
        .map(Room::from)
        .collect::<Vec<Room>>();

    Ok(rooms.as_response())
}

pub fn get_room(req: &mut Request) -> IronResult<Response> {
    let hotel_id = req.extensions
        .get::<Router>()
        .unwrap()
        .find("id")
        .map(|s| i32::from_str(s).unwrap())
        .expect("No Hotel ID in request");

    let room_number = req.extensions
        .get::<Router>()
        .unwrap()
        .find("number")
        .map(|s| i32::from_str(s).unwrap())
        .expect("No Room ID in request");

    info!("request GET /hotel/{}/room/{}", hotel_id, room_number);

    let conn = get_db_connection();
    let room = conn.query(&Room::select_builder()
                   .filter("HotelID = $1 and RoomNumber = $2")
                   .build(),
               &[&hotel_id, &room_number])
        .unwrap()
        .into_iter()
        .last()
        .map(Room::from);

    match room {
        Some(room) => Ok(room.as_response()),
        None => {
            Err(NotFoundError::from_str(format!("Room number {} not found in hotel {}",
                                               room_number,
                                               hotel_id))
                .into())
        }
    }
}

pub fn put_room(req: &mut Request) -> IronResult<Response> {
    let hotel_id = req.extensions
        .get::<Router>()
        .unwrap()
        .find("id")
        .map(|s| i32::from_str(s).unwrap())
        .expect("No Hotel ID in request");

    let conn = get_db_connection();
    let user = Authorizer::authorize_request(&conn, req)?;

    // Check authority
    if !user.roles.Owns.map_or(false, |owns| owns.contains(&hotel_id)) &&
       !user.roles.EmployedIn.map_or(false, |emp| emp.contains(&hotel_id)) {
        return Err(NotAuthorizedError::from_str(format!("Not owner or employee of hotel {}",
                                                        hotel_id))
            .into());
    }

    let new_room: NewRoom = request_body(req)?;

    info!("request PUT /hotel/{}/room/ {{ {:?} }}", hotel_id, new_room);

    let room = Room {
        HotelID: hotel_id,
        RoomNumber: new_room.RoomNumber,
        RoomLevel: new_room.RoomLevelID,
        PhotoSetID: new_room.PhotoSetID,
    };

    conn.execute(&Room::insert_query(), &room.insert_args())
        .unwrap();
    Ok(Response::with(StatusCode::Ok))
}

pub fn update_room(req: &mut Request) -> IronResult<Response> {
    let hotel_id = req.extensions
        .get::<Router>()
        .unwrap()
        .find("id")
        .map(|s| i32::from_str(s).unwrap())
        .expect("No Hotel ID in request");

    let room_number = req.extensions
        .get::<Router>()
        .unwrap()
        .find("number")
        .map(|s| i32::from_str(s).unwrap())
        .expect("No Room ID in request");

    let conn = get_db_connection();
    let user = Authorizer::authorize_request(&conn, req)?;

    // Check authority
    if !user.roles.Owns.map_or(false, |owns| owns.contains(&hotel_id)) &&
       !user.roles.EmployedIn.map_or(false, |emp| emp.contains(&hotel_id)) {
        return Err(NotAuthorizedError::from_str(format!("Not owner or employee of hotel {}",
                                                       hotel_id))
            .into());
    }

    let upd_room: UpdateRoom = request_body(req)?;

    info!("request POST /hotel/{}/room/{} {{ {:?} }}",
          hotel_id,
          room_number,
          upd_room);

    let mut values: Vec<&ToSql> = Vec::new();
    let mut update = Room::update_builder()
        .filter(format!("HotelID = {} and RoomNumber = {}", hotel_id, room_number));

    if let Some(room_lvl_id) = upd_room.RoomLevelID.as_ref() {
        values.push(room_lvl_id);
        update = update.set("RoomLevelID");
    }

    if let Some(photo_set_id) = upd_room.PhotoSetID.as_ref() {
        values.push(photo_set_id);
        update = update.set("PhotoSetID");
    }

    // Early exit if we got empty json
    if values.is_empty() {
        return Ok(Response::with(StatusCode::Ok));
    }

    conn.execute(&update.build(), &values)
        .unwrap();

    Ok(Response::with(StatusCode::Ok))
}

pub fn get_reviews(req: &mut Request) -> IronResult<Response> {
    let ofst = req.get_ref::<Params>()
        .unwrap()
        .find(&["offset"])
        .map(|val| i32::from_value(val).unwrap_or(0))
        .unwrap_or(0);

    let hotel_id = req.extensions
        .get::<Router>()
        .unwrap()
        .find("id")
        .map(|s| i32::from_str(s).unwrap())
        .expect("No Hotel ID in request");

    let cnt = req.extensions
        .get::<Router>()
        .unwrap()
        .find("cnt")
        .map(|s| i32::from_str(s).unwrap())
        .unwrap_or(i32::MAX);


    info!("request GET /hotel/{}/reviews/{}?offset={}",
          hotel_id,
          cnt,
          ofst);

    let conn = get_db_connection();
    let reviews = conn.query(&SelectQueryBuilder::default()
                   .columns("Review.*")
                   .from_tables("Review, Booking")
                   .filter("Booking.HotelID = $1 and Review.BookingID = Booking.ID")
                   .limit(cnt)
                   .offset(ofst)
                   .build(),
               &[&hotel_id])
        .unwrap()
        .into_iter()
        .map(Review::from)
        .collect::<Vec<Review>>();

    Ok(reviews.as_response())
}

pub fn get_employees(req: &mut Request) -> IronResult<Response> {
    let ofst = req.get_ref::<Params>()
        .unwrap()
        .find(&["offset"])
        .map(|val| i32::from_value(val).unwrap_or(0))
        .unwrap_or(0);

    let hotel_id = req.extensions
        .get::<Router>()
        .unwrap()
        .find("id")
        .map(|s| i32::from_str(s).unwrap())
        .expect("No Hotel ID in request");

    let cnt = req.extensions
        .get::<Router>()
        .unwrap()
        .find("cnt")
        .map(|s| i32::from_str(s).unwrap())
        .unwrap_or(i32::MAX);


    info!("request GET /hotel/{}/employees/{}?offset={}",
          hotel_id,
          cnt,
          ofst);

    let conn = get_db_connection();
    let persons = conn.query(&SelectQueryBuilder::default()
                   .columns("Person.*")
                   .from_tables("EmployedIn, Person")
                   .filter("EmployedIn.HotelID = $1 and EmployedIn.PersonID = Person.ID")
                   .limit(cnt)
                   .offset(ofst)
                   .build(),
               &[&hotel_id])
        .unwrap()
        .into_iter()
        .map(Person::from)
        .collect::<Vec<Person>>();

    let mut employees = Vec::with_capacity(persons.len());
    for person in persons.into_iter() {
        let roles = Authorizer::get_roles(&conn, person.ID)?;

        employees.push(Employee {
            Person: person,
            Roles: roles,
        });
    }

    Ok(employees.as_response())
}

pub fn fire_employee(req: &mut Request) -> IronResult<Response> {
    let hotel_id = req.extensions
        .get::<Router>()
        .unwrap()
        .find("id")
        .map(|s| i32::from_str(s).unwrap())
        .expect("No Hotel ID found in request");

    let emp_id = req.extensions
        .get::<Router>()
        .unwrap()
        .find("eid")
        .map(|s| i32::from_str(s).unwrap())
        .expect("No Employee ID found in request");

    let conn = get_db_connection();
    let user = Authorizer::authorize_request(&conn, req)?;

    info!("request DELETE /hotel/{}/employee/{}", hotel_id, emp_id);

    // Check authority in the hotel-to-update
    if !user.roles.Owns.map_or(false, |owns| owns.contains(&hotel_id)) {
        return Err(NotAuthorizedError::from_str(format!("Not owner or employee of hotel {}",
                                                       hotel_id))
            .into());
    }

    let emp_roles = Authorizer::get_roles(&conn, emp_id)?;

    if !emp_roles.EmployedIn.map_or(false, |employed_in| employed_in.contains(&emp_id)) {
        return Err(NotFoundError::from_str(format!("Person {} is not employed in {}",
                                                  emp_id,
                                                  hotel_id))
            .into());
    }

    conn.execute(&EmployedIn::delete_builder()
                     .filter("HotelID = $1, PersonID = $2")
                     .build(),
                 &[&hotel_id, &emp_id])
        .unwrap();

    Ok(Response::with(StatusCode::Ok))
}