use postgres::rows::Row;

auto_struct_from_row!(
    pub struct AccountInfo {
        pub ID: i32,
        pub Login: String,
        pub Name: String,
        pub Email: String
    }
);

pub mod procedure {
    use postgres::Connection;
    use postgres::types::{FromSql, ToSql};
    use chrono::NaiveDateTime;

    pub fn insert_booking(conn: &Connection,
                          ClientPersonID: i32,
                          HotelID: i32,
                          RoomNumber: i32,
                          BookingTime: NaiveDateTime,
                          ArrivalTime: NaiveDateTime,
                          DepartureTime: NaiveDateTime)
                          -> i32 {
        conn.query("insert_booking_and_return_id($1, $2, $3, $4, $5, $6)",
                   &[&ClientPersonID,
                     &HotelID,
                     &RoomNumber,
                     &BookingTime,
                     &ArrivalTime,
                     &DepartureTime])
            .unwrap()
            .get(0)
            .get(0)
    }
}