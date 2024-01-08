use crate::config::api::CONFIG;
use diesel::{r2d2, sqlite::SqliteConnection};

use super::api::Config;

#[allow(dead_code)]
pub type DbPool = r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>;
#[allow(dead_code)]
pub type DbError = Box<dyn std::error::Error + Send + Sync>;

impl Config {
    /// Establish a connection to MySQL database.
    pub fn establish_connection(config: &CONFIG) -> DbPool {
        let database_url = format!("{}://{}", config.db.dbms, config.db.name,);

        let manager = r2d2::ConnectionManager::<SqliteConnection>::new(&database_url);

        r2d2::Pool::builder().build(manager).expect(&format!("Error connecting to database {}", database_url))
    }
}
