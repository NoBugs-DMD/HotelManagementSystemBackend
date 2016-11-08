use postgres::rows::Row;

use super::*;
use super::builder::*;

auto_queries!(
    pub struct Hotel {
        pub ID: i32,
        pub OwnerPersonID: i32, 
        pub CityID: i32,
        pub RuleSetID: i32,
        pub Name: String,
        pub Description: String,
        pub Rating: Option<i32>,
        pub Stars: Option<i32>
    }
);

auto_queries!(
    pub struct Person {
        pub ID: i32,
        pub login: String,
        pub email: String,
        pub pass_hash: String
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
    pub struct Cleaner {
        pub PersonID: i32
    }
);

auto_queries!(
    pub struct Receptionist {
        pub PersonID: i32
    }
);

auto_queries!(
    pub struct Client {
        pub PersonID: i32,
        pub ClientLevelID: i32
    }
);

auto_queries!(
    pub struct EmployedIn {
        pub PersonID: i32,
        pub HotelID: i32
    }
);

auto_queries!(
    pub struct City {
        pub ID: i32,
        pub Name: String
    }
);