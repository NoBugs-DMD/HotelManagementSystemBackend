use chrono;
use iron::prelude::*;
use router::Router;
use hyper::status::StatusCode;

use super::request_body;
use ::api::authorization::*;
use ::proto::schema::*;
use ::proto::response::*;
use ::proto::error::*;
use ::db::schema::*;
use ::db::schemaext::*;
use ::db::*;

pub fn get_booking_by_id(req: &mut Request) -> IronResult<Response> {
    let conn = get_db_connection();
    let user = Authorizer::authorize_request(&conn, req)?;

    let booking_id = req.extensions
        .get::<Router>()
        .unwrap()
        .find("id")
        .unwrap();

    info!("request GET /api/booking/{} {{ id: {} }}",
          booking_id,
          user.id);

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
            return Err(NotFoundError::from_str(format!("No booking with id {}", booking_id)).into())
        }
    };

    let employee = conn.query(&EmployedIn::select_builder()
                   .filter("PersonID = $1")
                   .build(),
               &[&user.id])
        .unwrap()
        .into_iter()
        .map(EmployedIn::from)
        .last();

    let err_resp = NotAuthorizedError::from_str("Access denied, nor booking's owner nor hotel \
                                                 employee")
        .into();

    if user.id == booking.ClientPersonID {
        Ok(booking.as_response())
    } else if let Some(employee) = employee {
        if employee.HotelID == booking.HotelID {
            Ok(booking.as_response())
        } else {
            Err(err_resp)
        }
    } else {
        Err(err_resp)
    }
}

pub fn put_booking(req: &mut Request) -> IronResult<Response> {
    let conn = get_db_connection();

    let new_booking: NewBooking = request_body(req)?;
    let user = Authorizer::authorize_request(&conn, req)?;

    info!("request PUT /api/booking/ {{ id: {}, {:?} }}",
          user.id,
          new_booking);

    let client_id = if let Some(client_id) = new_booking.ClientPersonID {
        client_id
    } else {
        user.id
    };

    let receptionist = if client_id == user.id {
        None
    } else {
        let receptionist = conn.query(&SelectQueryBuilder::default()
                       .columns("EmployedIn.PersonID, EmployedInHotelID")
                       .from_tables("EmployedIn, Receptionist")
                       .filter("EmployedIn.PersonID = $1 and Receptionist.PersonID = \
                                EmployedIn.PersonID")
                       .build(),
                   &[&user.id])
            .unwrap()
            .into_iter()
            .map(EmployedIn::from)
            .last();

        if receptionist.is_none() {
            return Err(NotAuthorizedError::from_str("Only receptionist can make bookings on \
                                                        behalf of client")
                .into());
        }

        receptionist
    };

    let hotel_id = if let Some(hotel_id) = new_booking.HotelID {
        hotel_id
    } else if let Some(receptionist) = receptionist.as_ref() {
        receptionist.HotelID
    } else {
        return Err(NotAuthorizedError::from_str("Couldn't infer HotelID").into());
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