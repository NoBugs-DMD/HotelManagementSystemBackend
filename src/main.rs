#![feature(box_syntax)]
#![feature(trace_macros)]
#![feature(inclusive_range_syntax)]
#![feature(log_syntax)]
#![allow(non_snake_case)]

#[cfg(test)] extern crate rand;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate router;
#[macro_use] extern crate log;
extern crate env_logger;
extern crate postgres;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate hyper;
extern crate params;
extern crate iron;
extern crate cookie;
extern crate oven;
extern crate rustc_serialize;
extern crate dotenv;
extern crate chrono;

use iron::prelude::*;

#[macro_use]
mod proto;
mod db;
mod api;

fn main() {
    init_logging();

    let router = router! (
        signin:                   post   "/api/signin/"                  => api::authorization::signin,
        signup:                   post   "/api/signup/"                  => api::authorization::signup,
        
        city_get_cities:          get    "/api/city/"                    => api::city::get_cities,
        city_put_city:            put    "/api/city/"                    => api::city::put_city,
        
        account_get_all_bookings: get    "/api/account/bookings/"        => api::account::get_bookings,
        account_get_n_bookings:   get    "/api/account/bookings/:cnt"    => api::account::get_bookings,
        account_get_info:         get    "/api/account/"                 => api::account::get_account_info,
        account_update_info:      post   "/api/account/"                 => api::account::update_account_info,
        
        booking_get_booking:      get    "/api/booking/:id"              => api::booking::get_booking_by_id,
        booking_put_booking:      put    "/api/booking/"                 => api::booking::put_booking,
        
        hotel_get_all_hotels:     get    "/api/hotels/"                  => api::hotel::get_hotels,
        hotel_get_n_hotels:       get    "/api/hotels/:cnt"              => api::hotel::get_hotels,
        hotel_get_hotel_by_id:    get    "/api/hotel/:id"                => api::hotel::get_hotel,
        hotel_put_hotel:          put    "/api/hotel/"                   => api::hotel::put_hotel,
        hotel_update_hotel:       post   "/api/hotel/:id"                => api::hotel::update_hotel,               
        hotel_get_all_rooms:      get    "/api/hotel/:id/rooms/"         => api::hotel::get_rooms,               
        hotel_get_n_rooms:        get    "/api/hotel/:id/rooms/:cnt"     => api::hotel::get_rooms,
        hotel_get_room:           get    "/api/hotel/:id/room/:number"   => api::hotel::get_room,
        hotel_put_room:           put    "/api/hotel/:id/room/"          => api::hotel::put_room,
        hotel_update_room:        post   "/api/hotel/:id/room/:number"   => api::hotel::update_room,
        hotel_get_all_reviews:    get    "/api/hotel/:id/reviews/"       => api::hotel::get_reviews,
        hotel_get_n_reviews:      get    "/api/hotel/:id/reviews/:cnt"   => api::hotel::get_reviews,
        hotel_get_all_employees:  get    "/api/hotel/:id/employees/"     => api::hotel::get_employees,
        hotel_get_n_employees:    get    "/api/hotel/:id/employees/:cnt" => api::hotel::get_employees,
        hotel_del_employee:       delete "/api/hotel/:id/employee/:eid"  => api::hotel::fire_employee,
        hotel_get_ruleset:        get    "/api/hotel/:id/ruleset/"       => api::hotel::get_ruleset,
        hotel_update_ruleset:     post   "/api/hotel/:id/ruleset/"       => api::hotel::update_ruleset,    
    );

    let mut chain = Chain::new(router);

    // TODO __CHANGE__ key and load it from non-gited file.
    chain.link(oven::new(Vec::from(&b"f8f9eaf1ecdedff5e5b749c58115441e"[..])));

    // Get db connection from pool (will block until pool is ready)
    db::get_db_connection();

    Iron::new(chain).http("localhost:8080").unwrap();
}

use std::env;
use log::LogLevelFilter;
use env_logger::LogBuilder;

fn init_logging() {
    let mut builder = LogBuilder::new();
    builder.filter(None, LogLevelFilter::Info);

    if env::var("RUST_LOG").is_ok() {
        builder.parse(&env::var("RUST_LOG").unwrap());
    }

    builder.init().unwrap();
}
