use chrono::NaiveDateTime;
use diesel::prelude::*;
use dotenvy::dotenv;
use diesel::sqlite::SqliteConnection;
use std::env;
use diesel::connection::Connection;

pub mod models;
pub mod schema;


pub fn establish_connection() -> SqliteConnection{
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}


use self::models::NewVstsFeatureCompliance;
#[allow(non_snake_case)]
pub fn create_vsts_feature_compliance(conn: &mut SqliteConnection, FeatureID: i32, ReleaseName: String, IsCompliant: bool, NumberNonCompliantChildren: i32, LastCheckedDate: NaiveDateTime) {
    use crate::schema::VstsFeatureCompliance;
    
    let new_vsts_feature_compliance = NewVstsFeatureCompliance { 
        FeatureID,
        ReleaseName,
        IsCompliant,
        NumberNonCompliantChildren,
        LastCheckedDate
    };

    diesel::insert_into(VstsFeatureCompliance::table)
        .values(&new_vsts_feature_compliance)
        .execute(conn)
        .expect("Error saving new post");
}