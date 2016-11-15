use postgres::rows::Row;
use postgres::types::ToSql;
use chrono::NaiveDateTime;

use super::*;

auto_queries!(
    pub struct Person {
        pub ID: i32,
        pub Login: String,
        pub Name: String,
        pub Email: String,
        pub PassHash: String
    }
);

auto_queries!(
    pub struct Owner {
        pub PersonID: i32
    }
);

auto_queries!(
    pub struct Manager {
        pub PersonID: i32
    }
);

auto_queries!(
    pub struct Receptionist {
        pub PersonID: i32
    }
);

auto_queries!(
    pub struct Cleaner {
        pub PersonID: i32
    }
);

auto_queries!(
    pub struct RuleSet {
        pub ID: i32,
        pub ManagerPersonID: Option<i32>,
        pub Name: String,
        pub Body: String,
        pub IsDefault: bool
    }
);

auto_queries!(
    pub struct RoomLevel {
        pub Level: i32,
        pub RuleSetID: i32,
        pub LevelName: Option<String>,
        pub PerNight: i32
    }
);

auto_queries!(
    pub struct ClientLevel {
        pub BookingsAmount: i32,
        pub RuleSetID: i32,
        pub LevelName: Option<String>,
        pub DiscountPercentage: i32
    }
);

auto_queries!(
    pub struct Review {
        pub ID: i32,
        pub BookingID: i32,
        pub Body: String,
        pub LocationRate: i32,
        pub CleanlinessRate: i32,
        pub ServiceRate: i32,
        pub ValueForMoneyRate: i32,
        pub CreatedAt: NaiveDateTime
    }
);

auto_queries!(
    pub struct Client {
        pub PersonID: i32,
        pub ClientLevelID: i32
    }
);

auto_queries!(
    pub struct PhotoSet {
        pub ID: i32
    }
);

auto_queries!(
    pub struct Photo {
        pub ID: i32,
        pub Blob: Vec<u8>
    }
);

auto_queries!(
    pub struct PhotoSetPhotos {
        pub PhotoSetID: i32,
        pub PhotoID: i32
    }
);

auto_queries!(
    pub struct City {
        pub ID: i32,
        pub Name: String
    }
);

auto_queries!(
    pub struct Hotel {
        pub ID: i32,
        pub OwnerPersonID: i32,
        pub RuleSetID: i32,
        pub CityID: i32,
        pub PhotoSetID: Option<i32>,
        pub Name: String,
        pub Description: String,
        pub Rating: Option<i32>,
        pub Stars: Option<i32>
    }
);

auto_queries!(
    pub struct EmployedIn {
        pub PersonID: i32,
        pub HotelID: i32
    }
);

auto_queries!(
    pub struct Room {
        pub HotelID: i32,
        pub RoomNumber: i32,
        pub RoomLevel: i32,
        pub PhotoSetID: Option<i32>
    }
);

auto_queries!(
    pub struct Booking {
        pub ID: i32,
        pub ClientPersonID: i32,
        pub HotelID: i32,
        pub RoomNumber: i32,
        pub BookingTime: NaiveDateTime,
        pub ArrivalTime: NaiveDateTime,
        pub DepartureTime: NaiveDateTime,
        pub FullCost: i32,
        pub Paid: bool
    }
);

auto_queries!(
    pub struct MaintainedBy {
        pub BookingID: i32,
        pub ReceptionistPersonID: i32,
        pub MaintainedAt: NaiveDateTime
    }
);

auto_queries!(
    pub struct AssignedCleaning {
        pub ToCleanID: i32,
        pub CleanerPersonID: i32
    }
);