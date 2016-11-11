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

#[derive(Debug, Copy, Clone, Eq, PartialEq, RustcEncodable, RustcDecodable)]
pub struct Roles {
    pub Client: bool,
    pub Owner: bool,
    pub Manager: bool,
    pub Cleaner: bool,
    pub Receptionist: bool,
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct UpdateAccountInfoData {
    pub NewName: Option<String>,
    pub NewEmail: Option<String>,
    pub OldPassHash: Option<String>,
    pub NewPassHash: Option<String>,
}

#[derive(Debug, RustcDecodable)]
pub struct NewCity {
    pub Name: String,
}

#[derive(Debug, RustcDecodable)]
pub struct NewBooking {
    pub ClientPersonID: Option<i32>,
    pub HotelID: Option<i32>,
    pub RoomNumber: i32,
    pub ArrivalTime: NaiveDateTime,
    pub DepartureTime: NaiveDateTime,
}