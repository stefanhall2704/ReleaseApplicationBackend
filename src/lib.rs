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


use self::models::{NewVstsFeatureCompliance, VstsFeatureCompliance};
#[allow(non_snake_case)]
pub fn create_vsts_feature_compliance(conn: &mut PgConnection, FeatureID: i32, ReleaseName: String, IsCompliant: bool, NumberNonCompliantChildren: i32, LastCheckedDate: NaiveDateTime) -> VstsFeatureCompliance {
    use crate::schema::VstsFeatureCompliance;
    println!("lib");
    let new_vsts_feature_compliance = NewVstsFeatureCompliance { FeatureID, ReleaseName, IsCompliant, NumberNonCompliantChildren, LastCheckedDate };

    diesel::insert_into(VstsFeatureCompliance::table)
        .values(&new_vsts_feature_compliance)
        .get_result(conn)
        .expect("Error saving new post")
}

