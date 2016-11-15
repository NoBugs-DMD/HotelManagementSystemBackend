use iron::prelude::*;
use router::Router;
use hyper::status::StatusCode;
use postgres::types::ToSql;
use std::str::FromStr;
use std::i32;
use rustc_serialize::json;
use postgres::Connection;
use postgres::error::{Error, DbError, SqlState};

use super::request_body;
use super::decode_json;
use ::api::authorization::Authorizer;
use ::proto::response::*;
use ::proto::error::*;
use ::proto::schema::*;
use ::db::schema::{Hotel, RuleSet};
use ::db::*;

lazy_static!(
    pub static ref DEFAULT_RULESET_ID: i32 = {
        let conn = get_db_connection();
        let default_id = conn.query(&RuleSet::select_builder()
                              .filter("IsDefault = $1")
                              .build(),
                            &[&true])
            .unwrap()
            .into_iter()
            .map(RuleSet::from)
            .map(|set| set.ID)
            .last();

        let default_id = match default_id {
            Some(id) => id,
            None => {
                conn.execute(&RuleSet::insert_query(), &RuleSet {
                            ID: 0,
                            ManagerPersonID: None,
                            Name: "Default".to_owned(),
                            Body: json::encode(&*DEFAULT_RULESET).unwrap(),
                            IsDefault: true
                        }.insert_args())
                    .unwrap();
                conn.query(&RuleSet::select_builder()
                              .filter("IsDefault = $1")
                              .build(),
                            &[&true])
                    .unwrap()
                    .into_iter()
                    .map(RuleSet::from)
                    .map(|set| set.ID)
                    .last()
                    .unwrap()
            }
        };

        default_id
    };

    static ref DEFAULT_RULESET: Rules = Rules {
        RoomLevels: vec![
            RoomLevel {
                Name: Some("Ecomony".to_owned()),
                PerNight: 100,
                Level: 0
            },
            RoomLevel {
                Name: Some("Single".to_owned()),
                PerNight: 200,
                Level: 1
            },
            RoomLevel {
                Name: Some("Double".to_owned()),
                PerNight: 300,
                Level: 2
            },
            RoomLevel {
                Name: Some("Suit".to_owned()),
                PerNight: 400,
                Level: 3
            }
        ],
        ClientLevels: vec![
            ClientLevel {
                Name: None,
                Discount: 0,
                BookingsAmount: 0
            },
            ClientLevel {
                Name: None,
                Discount: 3,
                BookingsAmount: 10
            },
            ClientLevel {
                Name: None,
                Discount: 5,
                BookingsAmount: 20
            },
            ClientLevel {
                Name: None,
                Discount: 10,
                BookingsAmount: 50
            }
        ]
    };
);




pub fn process_rules(conn: &Connection, hotel_id: i32) -> ApiResult<()> {
    let hotel_ruleset_id = conn.query(&Hotel::select_builder()
                   .filter("ID = $1")
                   .build(),
               &[&hotel_id])
        .unwrap()
        .into_iter()
        .last()
        .map(Hotel::from)
        .map(|hotel| hotel.RuleSetID)
        .ok_or(box NotFoundError::from_str("No such Hotel") as Box<ApiError>)?;

    let ruleset: Rules = conn.query(&RuleSet::select_builder()
                   .filter("ID = $1")
                   .build(),
               &[&hotel_ruleset_id])
        .unwrap()
        .into_iter()
        .last()
        .map(RuleSet::from)
        .map(|rset| decode_json(&rset.Body))
        .ok_or(box NotFoundError::from_str("No such RuleSet") as Box<ApiError>)??;

    for room_level in ruleset.RoomLevels {
        let db_room_level = schema::RoomLevel {
            Level: room_level.Level,
            RuleSetID: hotel_ruleset_id,
            LevelName: room_level.Name,
            PerNight: room_level.PerNight,
        };

        match conn.execute(&schema::RoomLevel::insert_query(),
                          &db_room_level.insert_args())
        {
            Ok(_) => (),
            Err(Error::Db(db_err)) => match db_err.code {
                SqlState::UniqueViolation => (),
                ref state => panic!("{:?}", db_err)
            },
            Err(err) => panic!("{:?}", err)
        }
            
    }

    for client_level in ruleset.ClientLevels {
        let db_client_level = schema::ClientLevel {
            BookingsAmount: client_level.BookingsAmount,
            RuleSetID: hotel_ruleset_id,
            LevelName: client_level.Name,
            DiscountPercentage: client_level.Discount,
        };

        match conn.execute(&schema::ClientLevel::insert_query(),
                           &db_client_level.insert_args())
        {
            Ok(_) => (),
            Err(Error::Db(db_err)) => match db_err.code {
                SqlState::UniqueViolation => (),
                ref state => panic!("{:?}", db_err)
            },
            Err(err) => panic!("{:?}", err)
        }
    }

    Ok(())
}

#[derive(Debug, RustcDecodable, RustcEncodable)]
struct Rules {
    RoomLevels: Vec<RoomLevel>,
    ClientLevels: Vec<ClientLevel>,
}

#[derive(Debug, RustcDecodable, RustcEncodable)]
struct RoomLevel {
    Name: Option<String>,
    PerNight: i32,
    Level: i32,
}

#[derive(Debug, RustcDecodable, RustcEncodable)]
struct ClientLevel {
    Name: Option<String>,
    Discount: i32,
    BookingsAmount: i32,
}
