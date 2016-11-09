use postgres::rows::Row;

auto_struct_from_row!(
    pub struct AccountInfo {
        pub ID: i32,
        pub Login: String,
        pub Name: String,
        pub Email: String
    }
);