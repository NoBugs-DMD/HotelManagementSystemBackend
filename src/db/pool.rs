use postgres::params::IntoConnectParams;
use r2d2;
use r2d2_postgres::{TlsMode, PostgresConnectionManager};
use dotenv::dotenv;
use std;
use std::env;

lazy_static! {    
    static ref POOL: r2d2::Pool<PostgresConnectionManager> = {
        dotenv().ok();
        let connect_params = env::var("DATABASE_URL")
                .expect("DATABASE_URL must be set")
                .into_connect_params().unwrap();

        let config = r2d2::Config::default();
        let manager = PostgresConnectionManager::new(connect_params, TlsMode::None).unwrap();
        let pool = r2d2::Pool::new(config, manager).expect("Failed to create a pool");
        
        pool
    };
}

pub fn get_db_connection() -> r2d2::PooledConnection<PostgresConnectionManager> {
    loop {
        match POOL.get() {
            Err(r2d2::GetTimeout(_)) => std::thread::sleep(std::time::Duration::from_millis(10)),
            Ok(conn) => return conn,
        }
    }
}