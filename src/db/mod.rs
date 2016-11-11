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
    use rand;

    fn random_str() -> String {
        use rand::AsciiGenerator;
        use rand::Rng;

        rand::thread_rng()
            .gen_ascii_chars()
            .take(10)
            .collect()
    }

    #[test]
    fn auto_insert() {
        let conn = get_db_connection();
        let name = random_str();

        // Insert city with builder
        conn.execute(&City::insert_builder()
                .build(),
              &[&name])
            .unwrap();

        // Check insertion
        let cnt = conn.query("SELECT FROM City WHERE Name=$1", &[&name])
            .unwrap()
            .into_iter()
            .count();

        // Cleanup
        conn.execute("DELETE FROM City WHERE Name=$1;", &[&name])
            .unwrap();
        
        assert_eq!(cnt, 1);
    }

    #[test]
    fn auto_select() {
        let conn = get_db_connection();
        let name = random_str();

        // Insert city
        conn.execute("INSERT INTO City (Name) VALUES ($1);", &[&name])
            .unwrap();

        // Select with builder
        let cnt = conn.query(&City::select_builder()
                      .filter("Name=$1")
                      .build(),
                    &[&name])
            .unwrap()
            .into_iter()
            .count();

        // Cleanup
        conn.execute("DELETE FROM City WHERE Name=$1;", &[&name])
            .unwrap();

        assert_eq!(cnt, 1);
    }

    #[test]
    fn auto_delete() {
        let conn = get_db_connection();
        let name = random_str();

        // Insert city
        conn.execute("INSERT INTO City (Name) VALUES ($1);", &[&name])
            .unwrap();

        // Delete with builder
        conn.execute(&City::delete_builder()
                      .filter("Name=$1")
                      .build(),
                    &[&name])
            .unwrap();

        // Check deletion
        let cnt = conn.query("SELECT FROM City WHERE Name=$1", &[&name])
            .unwrap()
            .into_iter()
            .count();

        // Cleanup
        conn.execute("DELETE FROM City WHERE Name=$1;", &[&name])
            .unwrap();

        assert_eq!(cnt, 0);
    }

    #[test]
    fn auto_update() {
        let conn = get_db_connection();
        let name = random_str();
        let new_name = random_str();

        // Insert city
        conn.execute("INSERT INTO City (Name) VALUES ($1);", &[&name])
            .unwrap();

        // Update with builder
        let query = City::update_builder()
                      .filter(format!("Name='{}'", name))
                      .set("Name")
                      .build();
        println!("query: {:?}", query);
        conn.execute(&query,
                    &[&new_name])
            .unwrap();

        // Check updation
        let cnt = conn.query("SELECT FROM City WHERE Name=$1", &[&new_name])
            .unwrap()
            .into_iter()
            .count();

        // Cleanup
        conn.execute("DELETE FROM City WHERE Name=$1;", &[&name])
            .unwrap();
        conn.execute("DELETE FROM City WHERE Name=$1;", &[&new_name])
            .unwrap();

        assert_eq!(cnt, 1);
    }
}