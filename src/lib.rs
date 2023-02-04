use chrono::NaiveDateTime;
use dotenvy::dotenv;
use diesel::sqlite::SqliteConnection;
use std::env;
use diesel::connection::Connection;
use diesel::prelude::*;
use self::models::NewVstsFeatureCompliance;
use self::models::VstsFeatureCompliance as featureCompliance;
use crate::schema::VstsFeatureCompliance as featureComplianceSchema;

pub mod models;
pub mod schema;


pub fn establish_connection() -> SqliteConnection{
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[allow(non_snake_case)]
pub fn create_vsts_feature_compliance(conn: &mut SqliteConnection, feature_id: i32, release_name: String, is_compliant: bool, compliant_children: i32, last_checked_date: NaiveDateTime) {
    
    
    let new_vsts_feature_compliance = NewVstsFeatureCompliance { 
        FeatureID: feature_id,
        ReleaseName: release_name,
        IsCompliant: is_compliant,
        NumberNonCompliantChildren: compliant_children,
        LastCheckedDate: last_checked_date,
    };

    diesel::insert_into(featureComplianceSchema::table)
        .values(&new_vsts_feature_compliance)
        .execute(conn)
        .expect("Error saving new post");
}

#[allow(non_snake_case)]
pub fn get_feature_compliance_by_id(id: i32) -> Result<NewVstsFeatureCompliance, ()> {


    let connection = &mut establish_connection();
    let feature_compliance = featureComplianceSchema::table
    .filter(featureComplianceSchema::ID.eq(id))
    .first::<featureCompliance>(connection).unwrap();
    let data = NewVstsFeatureCompliance {
        FeatureID: feature_compliance.FeatureID,
        ReleaseName: feature_compliance.ReleaseName,
        IsCompliant: feature_compliance.IsCompliant,
        NumberNonCompliantChildren: feature_compliance.NumberNonCompliantChildren,
        LastCheckedDate: feature_compliance.LastCheckedDate
    };
    Ok(data)
}