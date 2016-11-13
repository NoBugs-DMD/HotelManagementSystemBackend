use rustc_serialize::json;
use router::Router;
use hyper::status::StatusCode;
use iron::prelude::*;
use params::{Params, FromValue};
use postgres::types::ToSql;
use std::str::FromStr;
use std::i32;

use super::request_body;
use ::api::authorization::Authorizer;
use ::proto::error::*;
use ::proto::response::*;
use ::proto::schema::*;
use ::db::schema::*;
use ::db::schemaext::*;
use ::db::*;


pub fn get_bookings(req: &mut Request) -> IronResult<Response> {
    let conn = get_db_connection();

    let client = authorize!(conn, req);

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

    info!("request GET /account/bookings {{ id: {}, cnt: {}, ofst: {} }}",
          client.id,
          cnt,
          ofst);

    let bookings = conn.query(&Booking::select_builder()
                   .filter("ClientPersonID = $1")
                   .limit(cnt)
                   .offset(ofst)
                   .build(),
               &[&client.id])
        .unwrap()
        .into_iter()
        .map(Booking::from)
        .collect::<Vec<Booking>>();

    Ok(ApiResponse::Ok(bookings).into())
}

pub fn get_account_info(req: &mut Request) -> IronResult<Response> {
    let conn = get_db_connection();
    let client = authorize!(conn, req);

    info!("request GET /account/ {{ id: {} }}", client.id);

    let info = conn.query(&Client::select_builder()
                   .columns("ID,Login,Name,Email")
                   .from_tables("Person")
                   .filter("ID = $1")
                   .build(),
               &[&client.id])
        .unwrap()
        .into_iter()
        .map(AccountInfo::from)
        .last()
        .unwrap();

    Ok(ApiResponse::Ok(info).into())
}

pub fn update_account_info(req: &mut Request) -> IronResult<Response> {
    let conn = get_db_connection();
    let client = authorize!(conn, req);
    let upd_info_data: UpdateAccountInfoData = decode_body!(req);

    info!("request POST /account/ {{ {:?} }}", upd_info_data);

    // Vector to store values that need an update
    let mut update = Person::update_builder().filter(format!("ID={}", client.id));
    let mut values: Vec<&ToSql> = Vec::with_capacity(3);

    // If user wants to update password, both OldPassHash and NewPassHash must be set
    if let Some(new_hash) = upd_info_data.NewPassHash.as_ref() {
        if let Some(old_hash) = upd_info_data.OldPassHash.as_ref() {
            // Try to query user with received old_hash
            let count = conn.query(&Person::select_builder()
                           .filter("ID = $1 and PassHash = $2")
                           .build(),
                       &[&client.id, &old_hash])
                .unwrap()
                .into_iter()
                .count();
            // If count is non-zero, there password is right
            if count != 0 {
                values.push(new_hash);
                update = update.set("PassHash")
            } else {
                return {
                    Ok(OldPasswordIsInvalidError::from_str("Old password is invalid")
                        .into_api_response()
                        .into())
                };
            }
        } else {
            return {
                Ok(IncompleteDataError::from_str("Missing OldPassHash")
                    .into_api_response()
                    .into())
            };
        }
    };

    if let Some(new_name) = upd_info_data.NewName.as_ref() {
        update = update.set("Name");
        values.push(new_name);
    }

    if let Some(new_email) = upd_info_data.NewEmail.as_ref() {
        update = update.set("Email");
        values.push(new_email);
    }

    // Early exit if we got empty json
    if values.is_empty() {
        return Ok(Response::with(StatusCode::Ok));
    }

    conn.execute(&update.build(), &values)
        .unwrap();
        
    Ok(Response::with(StatusCode::Ok))
}
