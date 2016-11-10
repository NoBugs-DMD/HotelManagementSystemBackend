// GET booking/?hotel=id&client=id&date[from]=timestamp&date[to]=timestamp
// GET booking/:id
// PUT booking/
// POST booking/:id

use chrono;
use iron::prelude::*;
use router::Router;
use rustc_serialize::json;
use hyper::status::StatusCode;

use super::request_body;
use ::api::authorization::*;
use ::proto::schema::*;
use ::proto::response::*;
use ::proto::error::*;
use ::db::schema::*;
use ::db::schemaext::*;
use ::db::*;

pub fn get_booking_by_id_handler(req: &mut Request) -> IronResult<Response> {
    let id = match Authorizer::authorize_request(req) {
        Ok(id) => id,
        Err(err) => return Ok(err.into_api_response().into()),
    };

    let booking_id = req.extensions
        .get::<Router>()
        .unwrap()
        .find("id")
        .unwrap();

    info!("request GET /api/booking/{} {{ id: {} }}", booking_id, id);

    let conn = get_db_connection();

    let booking = match conn.query(&Booking::select_builder()
                   .filter("ID = $1")
                   .build(),
               &[&booking_id])
        .unwrap()
        .into_iter()
        .map(Booking::from)
        .last() {
        Some(booking) => booking,
        None => {
            return Ok(NotFoundError::from_str(format!("No booking with id {}", booking_id))
                .into_api_response()
                .into());
        }
    };

    let employee = conn.query(&EmployedIn::select_builder()
                   .filter("PersonID = $1")
                   .build(),
               &[&id])
        .unwrap()
        .into_iter()
        .map(EmployedIn::from)
        .last();

    let err_resp = NotAuthorizedError::from_str("Access denied, nor booking's owner nor hotel \
                                                 employee")
        .into_api_response()
        .into();

    if id == booking.ClientPersonID {
        Ok(ApiResponse::Ok(booking).into())
    } else if let Some(employee) = employee {
        if employee.HotelID == booking.HotelID {
            Ok(ApiResponse::Ok(booking).into())
        } else {
            Ok(err_resp)
        }
    } else {
        Ok(err_resp)
    }
}

pub fn put_booking_handler(req: &mut Request) -> IronResult<Response> {
    let new_booking: NewBooking = match json::decode(&request_body(req)) {
        Ok(new_booking) => new_booking,
        Err(err) => return Ok(InvalidSchemaError::from(err).into_api_response().into()),
    };

    let id = match Authorizer::authorize_request(req) {
        Ok(id) => id,
        Err(err) => return Ok(err.into_api_response().into()),
    };

    info!("request PUT /api/booking/ {{ id: {}, {:?} }}", id, new_booking);

    let conn = get_db_connection();

    let client_id = if let Some(client_id) = new_booking.ClientPersonID {
        client_id
    } else {
        id
    };

    let receptionist = if client_id != id {
        match conn.query(&SelectQueryBuilder::default()
                       .columns("EmployedIn.PersonID, EmployedInHotelID")
                       .from_tables("EmployedIn, Receptionist")
                       .filter("EmployedIn.PersonID = $1 and Receptionist.PersonID = \
                                EmployedIn.PersonID")
                       .build(),
                   &[&id])
            .unwrap()
            .into_iter()
            .map(EmployedIn::from)
            .last() {
            Some(receptionist) => Some(receptionist),
            None => {
                return Ok(NotAuthorizedError::from_str("Only receptionist can make bookings on \
                                                        behalf of client")
                    .into_api_response()
                    .into())
            }
        }
    } else {
        None
    };

    let hotel_id = if let Some(hotel_id) = new_booking.HotelID {
        hotel_id
    } else if let Some(receptionist) = receptionist.as_ref() {
        receptionist.HotelID
    } else {
        return Ok(NotAuthorizedError::from_str("Couldn't infer HotelID")
            .into_api_response()
            .into());
    };

    let current_time = chrono::UTC::now().naive_local();
    let id = procedure::insert_booking(&conn,
                                       client_id,
                                       hotel_id,
                                       new_booking.RoomNumber,
                                       current_time,
                                       new_booking.ArrivalTime,
                                       new_booking.DepartureTime);

    if let Some(receptionist) = receptionist {
        conn.execute(&MaintainedBy::insert_query(),
                     &MaintainedBy {
                             BookingID: id,
                             ReceptionistPersonID: receptionist.PersonID,
                             MaintainedAt: current_time,
                         }
                         .insert_args())
            .unwrap();
    }

    Ok(Response::with(StatusCode::Ok))
}