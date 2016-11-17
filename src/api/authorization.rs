use std::collections::HashMap;
use std::sync::RwLock;
use std::sync::Arc;
use std::borrow::Borrow;

use hyper::header::CookiePair;
use postgres::Connection;
use iron::prelude::*;
use oven::prelude::*;

use super::request_body;
use ::proto::schema::*;
use ::proto::error::*;
use ::proto::response::*;
use ::db::schema::*;
use ::db::*;

pub type Token = String;

pub fn signin(req: &mut Request) -> IronResult<Response> {
    let signin_data: SigninData = request_body(req)?;

    info!("request POST /signin {{ {:?} }}", signin_data);

    let token = Authorizer::signin(&get_db_connection(), &signin_data)?;

    respond_with_roles_and_token(token)
}

pub fn signup(req: &mut Request) -> IronResult<Response> {
    let signup_data: SignupData = request_body(req)?;

    info!("request POST /signup {{ {:?} }}", signup_data);

    let token = Authorizer::signup(&get_db_connection(), &signup_data)?;

    respond_with_roles_and_token(token)
}

fn respond_with_roles_and_token(token: String) -> IronResult<Response> {
    let id = Authorizer::get_id(&token)?;
    let roles = Authorizer::get_roles(&get_db_connection(), id)?;

    let mut response: Response = roles.as_response();
    let mut cookie = CookiePair::new("token".to_string(), token.to_owned());

    // Nulling the path to tell browser to pass cookie for whole domain
    cookie.path = Some(String::new());

    response.set_cookie(cookie);
    Ok(response)
}

pub struct Authorized {
    pub id: i32,
    pub roles: Roles,
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

    pub fn authorize_request(conn: &Connection, req: &mut Request) -> ApiResult<Authorized> {
        let token_cookie = match req.get_cookie("token") {
            Some(tc) => tc,
            None => return Err(box NotAuthorizedError::from_str("No token found in request")),
        };

        let id = Self::get_id(&token_cookie.value)?;
        Ok(Authorized {
            id: id,
            roles: Self::get_roles(conn, id)?,
        })
    }

    fn get_id(token: &str) -> ApiResult<i32> {
        TOKEN_MAP.get(token)
            .ok_or(box NotAuthorizedError::from_str("Token has expired"))
    }

    pub fn get_roles(conn: &Connection, id: i32) -> ApiResult<Roles> {
        macro_rules! query_all_with_id {
            ($conn:ident, $table:ident) => (
                $conn.query(&$table::select_builder()
                                    .filter("PersonID = $1")
                                    .build(), &[&id]).unwrap()
            )
        }

        let owner = !query_all_with_id!(conn, Owner).is_empty();
        let manager = !query_all_with_id!(conn, Manager).is_empty();
        let cleaner = !query_all_with_id!(conn, Cleaner).is_empty();
        let receptionist = !query_all_with_id!(conn, Receptionist).is_empty();

        let owned_hotels = if owner {
            Some(conn.query(&Hotel::select_builder()
                           .columns("ID")
                           .filter("OwnerPersonID = $1")
                           .build(),
                       &[&id])
                .unwrap()
                .into_iter()
                .map(|row| row.get::<_, i32>("ID"))
                .collect())
        } else {
            None
        };

        let employed_in = if manager || cleaner || receptionist {
            Some(conn.query(&EmployedIn::select_builder()
                           .columns("HotelID")
                           .filter("PersonID = $1")
                           .build(),
                       &[&id])
                .unwrap()
                .into_iter()
                .map(|row| row.get::<_, i32>("HotelID"))
                .collect())
        } else {
            None
        };

        let roles = Roles {
            ID: id,
            Owner: owner,
            Owns: owned_hotels,
            Manager: manager,
            Cleaner: cleaner,
            Receptionist: receptionist,
            EmployedIn: employed_in,
        };

        debug!("{:?}", roles);

        Ok(roles)
    }
}

// Token storage
lazy_static! {
    static ref TOKEN_MAP: SyncMap<Token, i32> = SyncMap::new();
   // static ref ROLES_MAP: SyncMap<i32, Roles> = SyncMap::new();
}

use std::hash::Hash;
struct SyncMap<K: Eq + Hash, V> {
    map: Arc<RwLock<HashMap<K, V>>>,
}

impl<K, V> SyncMap<K, V>
    where K: Eq + Hash,
          V: Clone
{
    pub fn new() -> Self {
        SyncMap { map: Arc::new(RwLock::new(HashMap::new())) }
    }

    pub fn get<Q: ?Sized>(&self, token: &Q) -> Option<V>
        where K: Borrow<Q>,
              Q: Hash + Eq
    {
        self.map.read().unwrap().get(token.borrow()).cloned()
    }

    pub fn put(&self, token: K, id: V) {
        self.map.write().unwrap().insert(token, id);
    }
}