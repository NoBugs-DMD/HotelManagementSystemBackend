#[macro_use]
mod macros;
pub mod schema;
pub mod builder;
mod pool;

pub use self::pool::get_db_connection;

pub trait Insertable {
    fn insert_query() -> String; 
}

pub trait Queryable {
    fn select_builder() -> builder::SelectQueryBuilder;
}

pub trait Deletable {
    fn delete_builder() -> builder::DeleteQueryBuilder;
}

