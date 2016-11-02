extern crate postgres;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate rustless;
extern crate hyper;
extern crate iron;
extern crate rustc_serialize as serialize;
extern crate valico;

use valico::json_dsl;
use hyper::status::StatusCode;
use rustless::json::ToJson;
use rustless::{
    Application, Api, Nesting, Versioning
};

fn main() {
    let api = Api::build(|api| {
        // Specify API version
        api.prefix("api");

        api.post("signin", |endpoint| {
            endpoint.desc("Login user by login-password pair");
            endpoint.handle(|client, params| {
                unimplemented!()
            })
        })

        api.post("signup", |endpoint| {
            endpoint.desc("Sign-up user with provided info");
            endpoint.handle(|client, params| {
                unimplemented!()
            })
        })
    });

    let app = Application::new(api);
    iron::Iron::new(app).http("0.0.0.0:8080").unwrap();
}
