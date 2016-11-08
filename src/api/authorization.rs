use std::collections::HashMap;
use std::sync::RwLock;
use std::sync::Arc;
use std::borrow::Borrow;

use hyper::header::CookiePair;
use postgres::Connection;
use iron::prelude::*;
use oven::prelude::*;
use rustc_serialize::json;
use hyper::status::StatusCode; 
use std::io::Read;

use ::proto::schema::*;
use ::proto::error::*;
use ::db::schema::*;
use ::db::builder::*;
use ::db::*;

pub type Token = String;

new_api_error!(SigninError);
new_api_error!(SignupError);
new_api_error!(NotAuthorizedError);

pub fn signin_handler(req: &mut Request) -> IronResult<Response> {
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
    response.set_cookie(CookiePair::new("token".to_string(), token.to_owned()));

    Ok(response)
}

pub fn signup_handler(req: &mut Request) -> IronResult<Response> {
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
    response.set_cookie(CookiePair::new("token".to_string(), token.to_owned()));

    Ok(response)
}

struct Authorizer;
impl Authorizer {
    pub fn signin(conn: &Connection, signin_data: &SigninData) -> ApiResult<Token> {
        let query = Person::select_builder()
            .filter(&format!("login = $1 and pass_hash = $2"))
            .build();

        let rows = conn.query(&query, &[&signin_data.login, &signin_data.pass_md5]).unwrap();
        
        assert!(rows.len() <= 1, "Database is inconsistent");
        if rows.is_empty() {
            return Err(box SigninError::from("Login-password pair not found".to_owned()));
        }

        let person = Person::from(rows.get(0));
        let token = person.ID.to_string();
        TOKEN_MAP.put(token.clone(), person.ID);

        Ok(token)
    }

    pub fn signup(conn: &Connection, signup_data: &SignupData) -> ApiResult<Token> {
        match conn.execute(&Person::insert_query(), 
                           &[&signup_data.login, &signup_data.email, &signup_data.pass_md5]) {
            Ok(_) => (),
            Err(e) => return Err(box SignupError::from(format!("{}", e))),
        }
        
        Self::signin(conn, &SigninData {
            login: signup_data.login.clone(),
            pass_md5: signup_data.pass_md5.clone()
        })
    }

    pub fn authorize(token: &Token) -> ApiResult<i32> {
        TOKEN_MAP.get(token).ok_or(box NotAuthorizedError::from("Token has expired".to_owned()))
    }
}

// Token storage
lazy_static! {
    static ref TOKEN_MAP: TokenMap = TokenMap::new();
}

struct TokenMap {
    map: Arc<RwLock<HashMap<Token, i32>>>
}

impl TokenMap {
    pub fn new() -> TokenMap {
        TokenMap {
            map: Arc::new(RwLock::new(HashMap::new()))
        }
    }

    pub fn get<U: Borrow<Token>>(&self, token: U) -> Option<i32> {
        self.map.read().unwrap().get(token.borrow()).cloned()
    }

    pub fn put(&self, token: Token, id: i32) {
        self.map.write().unwrap().insert(token, id);
    }
}