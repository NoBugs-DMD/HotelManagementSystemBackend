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
use ::proto::response::*;
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
        Err(json_err) => return Ok(InvalidSchemaError::from(json_err).into_api_response().into())
    };

    println!("Signin request {:?}", signin_data);
    
    let token = match Authorizer::signin(&get_db_connection(), &signin_data) {
        Ok(token) => token,
        Err(err) => return Ok(err.into_api_response().into())
    };

    println!("token: {}", token);

    Ok(respond_with_roles_and_token(token))
}

pub fn signup_handler(req: &mut Request) -> IronResult<Response> {
    let mut buffer = String::with_capacity(128);
    req.body.read_to_string(&mut buffer).unwrap();

    let signup_data: SignupData = match json::decode(&buffer) {
        Ok(sd) => sd,
        Err(json_err) => return Ok(InvalidSchemaError::from(json_err).into_api_response().into())
    };

    println!("Signup request: {:?}", signup_data);
    
    let token = match Authorizer::signup(&get_db_connection(), &signup_data) {
        Ok(token) => token,
        Err(err) => return Ok(err.into_api_response().into())
    };

    Ok(respond_with_roles_and_token(token))
}

fn respond_with_roles_and_token(token: String) -> Response {
    let roles = match Authorizer::get_roles(&get_db_connection(), &token) {
        Ok(roles) => roles,
        Err(err) => return err.into_api_response().into()   
    };

    let mut response: Response = ApiResponse::Ok(roles).into();
    response.set_cookie(CookiePair::new("token".to_string(), token.to_owned()));
    response
}

struct Authorizer;
impl Authorizer {
    pub fn signin(conn: &Connection, signin_data: &SigninData) -> ApiResult<Token> {
        let query = Person::select_builder()
            .filter("Login = $1 and PassHash = $2")
            .build();

        print!("query: {:?}", query);

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

    pub fn get_id(token: &Token) -> ApiResult<i32> {
        TOKEN_MAP.get(token)
                 .ok_or(box NotAuthorizedError::from("Token has expired".to_owned()))
    }

    pub fn get_roles(conn: &Connection, token: &Token) -> ApiResult<Roles> {
        let id = Self::get_id(token)?;
        
        if let Some(roles) = ROLES_MAP.get(&id) {
            return Ok(roles);
        }

        macro_rules! query_all_with_id {
            ($table:ident) => (
                conn.query(&$table::select_builder()
                                    .filter("PersonID = $1")
                                    .build(), &[&id]).unwrap();
            )
        }

        let conn = get_db_connection();

        let clients = query_all_with_id!(Client);
        let owners = query_all_with_id!(Owner);
        let managers = query_all_with_id!(Manager);
        let cleaners = query_all_with_id!(Cleaner);
        let receptionists = query_all_with_id!(Receptionist);

        let roles = Roles {
            client: !clients.is_empty(),
            owner:  !owners.is_empty(),
            manager: !managers.is_empty(),
            cleaner: !cleaners.is_empty(),
            receptionist: !receptionists.is_empty(), 
        };

        ROLES_MAP.put(id, roles.clone());
        Ok(roles)
    } 
}

// Token storage
lazy_static! {
    static ref TOKEN_MAP: SyncMap<Token, i32> = SyncMap::new();
    static ref ROLES_MAP: SyncMap<i32, Roles> = SyncMap::new();
}


use std::hash::Hash;
struct SyncMap<K: Eq + Hash, V> {
    map: Arc<RwLock<HashMap<K, V>>>
}

impl<K, V> SyncMap<K, V> 
    where K: Eq + Hash, V: Copy + Clone
{
    pub fn new() -> Self {
        SyncMap {
            map: Arc::new(RwLock::new(HashMap::new()))
        }
    }

    pub fn get<U: Borrow<K>>(&self, token: U) -> Option<V> {
        self.map.read().unwrap().get(token.borrow()).cloned()
    }

    pub fn put(&self, token: K, id: V) {
        self.map.write().unwrap().insert(token, id);
    }
}