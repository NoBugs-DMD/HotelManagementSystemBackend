#[macro_use]
mod macros;
pub mod schema;
pub mod schemaext;
mod builder;
mod pool;

pub use self::pool::get_db_connection;
pub use self::builder::*;

pub trait Insertable {
    fn insert_builder<'a>() -> builder::InsertQueryBuilder<'a>;
    fn insert_query() -> String {
        Self::insert_builder().build()
    }
}

pub trait Queryable {
    fn select_builder<'a>() -> builder::SelectQueryBuilder<'a>;
}

pub trait Deletable {
    fn delete_builder<'a>() -> builder::DeleteQueryBuilder<'a>;
}

pub trait Updatable {
    fn update_builder<'a>() -> builder::UpdateQueryBuilder<'a>;
}
