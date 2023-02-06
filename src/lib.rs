use chrono::NaiveDateTime;
use dotenvy::dotenv;
use diesel::sqlite::SqliteConnection;
use std::env;
use diesel::connection::Connection;
use diesel::prelude::*;


use self::models::ApplicationTeam as application_team;
use self::models::NewApplicationTeam;
use self::schema::ApplicationTeam as application_team_schema;


use self::models::NewVstsFeatureCompliance;
use self::models::VstsFeatureCompliance as featureCompliance;
use self::schema::VstsFeatureCompliance as featureComplianceSchema;

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


#[allow(non_snake_case)]
pub fn create_application_team(conn: &mut SqliteConnection, name: String, is_active: Option<bool>, source_control_team_id: Option<String> ) {
    // let optional_int = is_active.unwrap();
    let application_team = NewApplicationTeam { Name: name, IsActive: is_active, SourceControlTeamID: source_control_team_id };


    diesel::insert_into(application_team_schema::table)
        .values(&application_team)
        .execute(conn)
        .expect("Error saving new post");
}


pub fn get_application_team_by_id(id: i32) -> Result<NewApplicationTeam, ()> {


    let connection = &mut establish_connection();

    let application_team_db = application_team_schema::table
    .filter(application_team_schema::ID.eq(id))
    .first::<application_team>(connection).unwrap();

    let data = NewApplicationTeam { Name: application_team_db.Name, IsActive: application_team_db.IsActive, SourceControlTeamID: application_team_db.SourceControlTeamID };
    Ok(data)
}


pub fn update_application_team(conn: &mut SqliteConnection, id: i32, name: String, is_active: Option<bool>, source_control_team_id: Option<String>) {
    let application_team_db = NewApplicationTeam { Name: name, IsActive: is_active, SourceControlTeamID: source_control_team_id };


    diesel::update(application_team_schema::table.find(id))
        .set((
            application_team_schema::Name.eq(application_team_db.Name),
            application_team_schema::IsActive.eq(application_team_db.IsActive),
            application_team_schema::SourceControlTeamID.eq(application_team_db.SourceControlTeamID)
        ))
        .execute(conn)
        .expect("Error saving new post");

}


pub fn delete_application_team(conn: &mut SqliteConnection, id: i32) {
    diesel::delete(application_team_schema::table.find(id))
        .execute(conn)
        .expect("Error saving new post");

}