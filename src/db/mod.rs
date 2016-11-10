#[macro_use]
mod macros;
pub mod schema;
pub mod schemaext;
mod builder;
mod pool;

pub use self::pool::get_db_connection;
pub use self::builder::*;

use postgres::types::ToSql;

pub trait Insertable {
    fn insert_builder<'a>() -> builder::InsertQueryBuilder<'a>;
    fn insert_query() -> String {
        Self::insert_builder().build()
    }
    fn insert_args(&self) -> Vec<&ToSql>;
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

#[cfg(test)]
mod tests {
    use ::db::schema::*;
    use ::db::*;

    #[test]
    fn auto_insert() {
        let conn = get_db_connection();

        // Insert city with builder
        conn.execute(&City::insert_builder()
                .build(),
              &[&"insert_test_city"])
            .unwrap();

        // Check insertion
        let cnt = conn.query("SELECT FROM City WHERE Name=$1", &[&"insert_test_city"])
            .unwrap()
            .into_iter()
            .count();

        // Cleanup
        conn.execute("DELETE FROM City WHERE Name=$1;", &[&"insert_test_city"])
            .unwrap();
        
        assert_eq!(cnt, 1);
    }

    #[test]
    fn auto_select() {
        let conn = get_db_connection();

        // Insert city
        conn.execute("INSERT INTO City (Name) VALUES ($1);", &[&"select_test_city"])
            .unwrap();

        // Select with builder
        let cnt = conn.query(&City::select_builder()
                      .filter("Name=$1")
                      .build(),
                    &[&"select_test_city"])
            .unwrap()
            .into_iter()
            .count();

        // Cleanup
        conn.execute("DELETE FROM City WHERE Name=$1;", &[&"select_test_city"])
            .unwrap();

        assert_eq!(cnt, 1);
    }

    #[test]
    fn auto_delete() {
        let conn = get_db_connection();

        // Insert city
        conn.execute("INSERT INTO City (Name) VALUES ($1);", &[&"delete_test_city"])
            .unwrap();

        // Delete with builder
        conn.execute(&City::delete_builder()
                      .filter("Name=$1")
                      .build(),
                    &[&"delete_test_city"])
            .unwrap();

        // Check deletion
        let cnt = conn.query("SELECT FROM City WHERE Name=$1", &[&"delete_test_city"])
            .unwrap()
            .into_iter()
            .count();

        // Cleanup
        conn.execute("DELETE FROM City WHERE Name=$1;", &[&"delete_test_city"])
            .unwrap();

        assert_eq!(cnt, 0);
    }

    #[test]
    fn auto_update() {
        let conn = get_db_connection();

        // Insert city
        conn.execute("INSERT INTO City (Name) VALUES ($1);", &[&"update_test_city"])
            .unwrap();

        // Update with builder
        conn.execute(&City::update_builder()
                      .filter("Name=$1")
                      .set("Name=$2")
                      .build(),
                    &[&"update_test_city", &"update_test_city2"])
            .unwrap();

        // Check updation
        let cnt = conn.query("SELECT FROM City WHERE Name=$1", &[&"update_test_city2"])
            .unwrap()
            .into_iter()
            .count();

        // Cleanup
        conn.execute("DELETE FROM City WHERE Name=$1;", &[&"update_test_city"])
            .unwrap();
        conn.execute("DELETE FROM City WHERE Name=$1;", &[&"update_test_city2"])
            .unwrap();

        assert_eq!(cnt, 1);
    }
}