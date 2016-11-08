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
        signin:     post   "/api/singin/" => api::authorization::signin_handler,
        signup:     post   "/api/signup/" => api::authorization::signup_handler,
        get_cities: get    "/api/city/"   => api::city::get_cities_handler,
        put_city:   put    "/api/city/"   => api::city::put_city_handler,
    );
    
    let mut chain = Chain::new(router);

    // TODO __CHANGE__ key and load it from non-gited file.
    chain.link(oven::new(Vec::from(&b"f8f9eaf1ecdedff5e5b749c58115441e"[..])));
    
    Iron::new(chain).http("localhost:8080").unwrap();
}

