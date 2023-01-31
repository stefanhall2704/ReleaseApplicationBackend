use chrono::NaiveDateTime;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;


pub mod models;
pub mod schema;


pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}


use self::models::{NewApplicationConnection, ApplicationConnection};

pub fn create_post(conn: &mut PgConnection, FromApplicationID: i32, ToApplicationID: i32, RequiresCert: bool, RequiresKey: bool, RequiresAzureADAuth: bool, ConnectionType: String, DateAdded: NaiveDateTime) -> ApplicationConnection {
    use crate::schema::ApplicationConnection;

    let new_app_connection = NewApplicationConnection { FromApplicationID, ToApplicationID, RequiresCert, RequiresKey, RequiresAzureADAuth, ConnectionType, DateAdded };

    diesel::insert_into(ApplicationConnection::table)
        .values(&new_app_connection)
        .get_result(conn)
        .expect("Error saving new post")
}

