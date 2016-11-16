use chrono::NaiveDateTime;

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