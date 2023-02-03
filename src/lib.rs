use chrono::NaiveDateTime;
use dotenvy::dotenv;
use diesel::sqlite::SqliteConnection;
use std::env;
use diesel::connection::Connection;
use diesel::prelude::*;
use self::models::NewVstsFeatureCompliance;
use self::models::VstsFeatureCompliance as featureCompliance;
use crate::schema::VstsFeatureCompliance;

pub mod models;
pub mod schema;


pub fn establish_connection() -> SqliteConnection{
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[allow(non_snake_case)]
pub fn create_vsts_feature_compliance(conn: &mut SqliteConnection, FeatureID: i32, ReleaseName: String, IsCompliant: bool, NumberNonCompliantChildren: i32, LastCheckedDate: NaiveDateTime) {
    
    
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

#[allow(non_snake_case)]
pub fn get_feature_compliance_by_id(id: i32) -> Result<Result<models::VstsFeatureCompliance, diesel::result::Error>, ()> {


    let connection = &mut establish_connection();
    // let results = VstsFeatureCompliance
    //     .filter(ID.eq(id))
    //     .limit(1)
    //     .load::<featureCompliance>(connection)
    //     .expect("Error loading posts");
    // let result = results.first().unwrap();
    // Ok(Some(result[0].clone()))
    let feature_compliance = VstsFeatureCompliance::table
    .filter(VstsFeatureCompliance::ID.eq(id))
    .get_result::<featureCompliance>(connection);
    Ok(feature_compliance)
    // println!("Displaying {} posts", results.len());
    // for result in results {
    //     println!("{}", result.ReleaseName);
    // }
}
// use self::schema::VstsFeatureCompliance::dsl::*;
// use self::models::VstsFeatureCompliance as featureCompliance;
// pub fn get_feature_compliance_by_id(id: i32) -> Result<featureCompliance, diesel::result::Error>{


//     let connection = &mut establish_connection();
//     // let results = VstsFeatureCompliance
//     //     .filter(ID.eq(id))
//     //     .limit(1)
//     //     .load::<featureCompliance>(connection)
//     //     .expect("Error loading posts");
//     //let new_result = <VstsFeatureCompliance as diesel::associations::HasTable>::table.find(id).get_result::<featureCompliance>(connection)?;
//     // let new_result = <VstsFeatureCompliance as diesel::associations::HasTable>::table.select.find(id).get_result::<featureCompliance>(connection)?;
//     let result = <VstsFeatureCompliance as diesel::associations::HasTable>::table.select(VstsFeatureCompliance::all_columns).filter(VstsFeatureCompliance::ID.eq(id)).first::<featureCompliance>(connection)?;
//     Ok(result)
//     // println!("Displaying {} posts", results.len());
//     // for result in results {
//     //     println!("{}", result.ReleaseName);
//     // }
// }