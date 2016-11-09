use std::collections::HashMap;
use std::sync::RwLock;
use std::sync::Arc;
use std::borrow::Borrow;

use hyper::header::CookiePair;
use postgres::Connection;
use iron::prelude::*;
use oven::prelude::*;
use rustc_serialize::json;

use super::request_body;
use ::proto::schema::*;
use ::proto::error::*;
use ::proto::response::*;
use ::db::schema::*;
use ::db::*;

pub type Token = String;

new_api_error!(SigninError);
new_api_error!(SignupError);
new_api_error!(NotAuthorizedError);

pub fn signin_handler(req: &mut Request) -> IronResult<Response> {
    let signin_data: SigninData = match json::decode(&request_body(req)) {
        Ok(sd) => sd,
        Err(json_err) => return Ok(InvalidSchemaError::from(json_err).into_api_response().into()),
    };

    info!("request POST /signin {{ {:?} }}", signin_data);

    let token = match Authorizer::signin(&get_db_connection(), &signin_data) {
        Ok(token) => token,
        Err(err) => return Ok(err.into_api_response().into()),
    };

    Ok(respond_with_roles_and_token(token))
}

pub fn signup_handler(req: &mut Request) -> IronResult<Response> {
    let signup_data: SignupData = match json::decode(&request_body(req)) {
        Ok(sd) => sd,
        Err(json_err) => return Ok(InvalidSchemaError::from(json_err).into_api_response().into()),
    };

    info!("request POST /signup {{ {:?} }}", signup_data);

    let token = match Authorizer::signup(&get_db_connection(), &signup_data) {
        Ok(token) => token,
        Err(err) => return Ok(err.into_api_response().into()),
    };

    Ok(respond_with_roles_and_token(token))
}

fn respond_with_roles_and_token(token: String) -> Response {
    let roles = match Authorizer::get_roles(&get_db_connection(), &token) {
        Ok(roles) => roles,
        Err(err) => return err.into_api_response().into(),   
    };

    let mut response: Response = ApiResponse::Ok(roles).into();
    let mut cookie = CookiePair::new("token".to_string(), token.to_owned());

    // Nulling the path to tell browser to pass cookie for whole domain
    cookie.path = Some(String::new());

    response.set_cookie(cookie);
    response
}

pub struct Authorizer;
impl Authorizer {
    pub fn signin(conn: &Connection, signin_data: &SigninData) -> ApiResult<Token> {
        let query = Person::select_builder()
            .filter("Login = $1 and PassHash = $2")
            .build();

        let rows = conn.query(&query, &[&signin_data.Login, &signin_data.PassHash]).unwrap();

        assert!(rows.len() <= 1, "Database is inconsistent");
        if rows.is_empty() {
            return Err(box SigninError::from_str("Login-password pair not found"));
        }

        let person = Person::from(rows.get(0));
        let token = person.ID.to_string();
        TOKEN_MAP.put(token.clone(), person.ID);

        Ok(token)
    }

    pub fn signup(conn: &Connection, signup_data: &SignupData) -> ApiResult<Token> {
        match conn.execute(&Person::insert_query(),
                           &[&signup_data.Login,
                             &signup_data.Name,
                             &signup_data.Email,
                             &signup_data.PassHash]) {
            Ok(_) => (),
            Err(e) => return Err(box SignupError::from_str(format!("{}", e))),
        }

        Self::signin(conn, &SigninData {
            Login: signup_data.Login.clone(),
            PassHash: signup_data.PassHash.clone(),
        })
    }

    pub fn authorize_request(req: &mut Request) -> ApiResult<i32> {
        let token_cookie = match req.get_cookie("token") {
            Some(tc) => tc,
            None => {
                return Err(box NotAuthorizedError::from_str("No token found in request"))
            }
        };

        Self::get_id(&token_cookie.value)
    }

    pub fn get_id(token: &str) -> ApiResult<i32> {
        TOKEN_MAP.get(token)
            .ok_or(box NotAuthorizedError::from_str("Token has expired"))
    }

    pub fn get_roles(conn: &Connection, token: &str) -> ApiResult<Roles> {
        let id = Self::get_id(token)?;

        if let Some(roles) = ROLES_MAP.get(&id) {
            return Ok(roles);
        }

        macro_rules! query_all_with_id {
            ($conn:ident, $table:ident) => (
                $conn.query(&$table::select_builder()
                                    .filter("PersonID = $1")
                                    .build(), &[&id]).unwrap();
            )
        }

        let clients = query_all_with_id!(conn, Client);
        let owners = query_all_with_id!(conn, Owner);
        let managers = query_all_with_id!(conn, Manager);
        let cleaners = query_all_with_id!(conn, Cleaner);
        let receptionists = query_all_with_id!(conn, Receptionist);

        let roles = Roles {
            Client:       !clients.is_empty(),
            Owner:        !owners.is_empty(),
            Manager:      !managers.is_empty(),
            Cleaner:      !cleaners.is_empty(),
            Receptionist: !receptionists.is_empty(),
        };

        ROLES_MAP.put(id, roles);
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
    map: Arc<RwLock<HashMap<K, V>>>,
}

impl<K, V> SyncMap<K, V>
    where K: Eq + Hash,
          V: Copy + Clone
{
    pub fn new() -> Self {
        SyncMap { map: Arc::new(RwLock::new(HashMap::new())) }
    }

    pub fn get<Q: ?Sized>(&self, token: &Q) -> Option<V> 
        where K: Borrow<Q>, Q: Hash + Eq    
    {
        self.map.read().unwrap().get(token.borrow()).cloned()
    }

    pub fn put(&self, token: K, id: V) {
        self.map.write().unwrap().insert(token, id);
    }
}