use chrono::NaiveDateTime;
use postgres::Connection;
use ::db::schema::Person;
use ::db::schema::Room;
use ::db::*;

#[derive(Debug, RustcDecodable)]
pub struct SigninData {
    pub Login: String,
    pub PassHash: String,
}

#[derive(Debug, RustcDecodable)]
pub struct SignupData {
    pub Login: String,
    pub Name: String,
    pub Email: String,
    pub PassHash: String,
}

#[derive(Debug, Clone, Eq, PartialEq, RustcEncodable, RustcDecodable)]
pub struct Roles {
    pub ID: i32,
    pub Owner: bool,
    pub Owns: Option<Vec<i32>>,
    pub Manager: bool,
    pub Cleaner: bool,
    pub Receptionist: bool,
    pub EmployedIn: Option<Vec<i32>>,
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct UpdateAccountInfoData {
    pub NewName: Option<String>,
    pub NewEmail: Option<String>,
    pub OldPassHash: Option<String>,
    pub NewPassHash: Option<String>,
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct NewCity {
    pub Name: String,
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct NewBooking {
    pub ClientPersonID: Option<i32>,
    pub HotelID: Option<i32>,
    pub RoomNumber: i32,
    pub ArrivalTime: NaiveDateTime,
    pub DepartureTime: NaiveDateTime,
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct NewHotel {
    pub CityID: i32,
    pub Name: String,
    pub Description: String,
    pub Stars: Option<i32>,
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct UpdateHotel {
    pub RuleSetID: Option<i32>,
    pub Name: Option<String>,
    pub Description: Option<String>,
    pub PhotoSetID: Option<i32>,
    pub Stars: Option<i32>,
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct NewRoom {
    pub RoomNumber: i32,
    pub RoomLevelID: i32,
    pub PhotoSetID: Option<i32>
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct UpdateRoom {
    pub RoomLevelID: Option<i32>,
    pub PhotoSetID: Option<i32>
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Employee {
    pub Person: Person,
    pub Roles: Roles,
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct NewRuleSet {
    pub Name: String,
    pub Body: String,
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct UpdateRuleSet {
    pub Name: Option<String>,
    pub Body: Option<String>,
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Range<T: Ord> {
    pub from: T,
    pub to: T
} 

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct SearchRequest {
    pub CityID: i32,
    pub DateTime: Option<Range<NaiveDateTime>>,
    pub Rating: Option<Range<i32>>,
    pub Stars: Option<Range<i32>>,
    pub Price: Option<Range<i32>>,
    pub HotelID: Option<i32>,
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct PricedRoom {
    pub Room: Room,
    pub Price: i32,
}

impl PricedRoom {
    pub fn from_room(conn: &Connection, room: Room, client_id: i32) -> PricedRoom {
        use postgres::types::FromSql;
        let price = conn.query("SELECT room_price($1, $2, $3)", &[&room.RoomLevel, &room.HotelID, &client_id])
            .unwrap()
            .into_iter()
            .last()
            .map(|row| row.get(0))
            .unwrap();
            
        PricedRoom {
            Room: room,
            Price: price
        }
    }
}



