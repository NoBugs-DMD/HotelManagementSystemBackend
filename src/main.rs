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

use iron::prelude::*;
use oven::prelude::*;
use rustc_serialize::json;
use hyper::status::StatusCode; 
use std::io::Read;

#[macro_use]
mod proto;
mod db;
mod authorization;

use proto::error::*;
use proto::schema::*;
use authorization::*;

use db::*;

fn main() {
    let router = router! (
        signin:   post   "/api/singin/" => signin_handler,
        signup:   post   "/api/signup/" => signup_handler,
    );
    
    let mut chain = Chain::new(router);

    // TODO __CHANGE__ key and load it from non-gited file.
    chain.link(oven::new(Vec::from(&b"f8f9eaf1ecdedff5e5b749c58115441e"[..])));
    
    Iron::new(chain).http("localhost:8080").unwrap();
}

fn signin_handler(req: &mut Request) -> IronResult<Response> {
    let mut buffer = String::with_capacity(128);
    req.body.read_to_string(&mut buffer).unwrap();

    let signin_data: SigninData = match json::decode(&buffer) {
        Ok(sd) => sd,
        Err(json_err) => return InvalidSchemaError::from(json_err).into_api_response().into()
    };

    println!("Signin request {:?}", signin_data);
    
    let token = match Authorizer::signin(&get_db_connection(), &signin_data) {
        Ok(token) => token,
        Err(err) => return err.into_api_response().into()
    };

    let mut response = Response::with(StatusCode::Ok);
    response.set_cookie(hyper::header::CookiePair::new("token".to_string(), token.to_owned()));

    Ok(response)
}

fn signup_handler(req: &mut Request) -> IronResult<Response> {
    let mut buffer = String::with_capacity(128);
    req.body.read_to_string(&mut buffer).unwrap();

    let signup_data: SignupData = match json::decode(&buffer) {
        Ok(sd) => sd,
        Err(json_err) => return InvalidSchemaError::from(json_err).into_api_response().into()
    };

    println!("Signup request: {:?}", signup_data);
    
    let token = match Authorizer::signup(&get_db_connection(), &signup_data) {
        Ok(token) => token,
        Err(err) => return err.into_api_response().into()
    };

    let mut response = Response::with(StatusCode::Ok);
    response.set_cookie(hyper::header::CookiePair::new("token".to_string(), token.to_owned()));

    Ok(response)
}