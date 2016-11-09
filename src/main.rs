#![feature(box_syntax)]
#![feature(trace_macros)]
#![feature(inclusive_range_syntax)]
#![feature(log_syntax)]

extern crate postgres;
extern crate r2d2;
extern crate r2d2_postgres;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate router;
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
    let router = router! (
        signin:               post   "/api/signin/"               => api::authorization::signin_handler,
        signup:               post   "/api/signup/"               => api::authorization::signup_handler,
        city_get_cities:      get    "/api/city/"                 => api::city::get_cities_handler,
        city_put_city:        put    "/api/city/"                 => api::city::put_city_handler,
        account_get_bookings: get    "/api/account/bookings/:cnt" => api::account::get_bookings_handle,
    );
    
    let mut chain = Chain::new(router);

    // TODO __CHANGE__ key and load it from non-gited file.
    chain.link(oven::new(Vec::from(&b"f8f9eaf1ecdedff5e5b749c58115441e"[..])));
    
    // Get db connection from pool (will block until pool is ready)
    db::get_db_connection();

    Iron::new(chain).http("localhost:8080").unwrap();
}

