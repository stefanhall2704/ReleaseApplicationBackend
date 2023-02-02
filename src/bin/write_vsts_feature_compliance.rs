use chrono::{NaiveDate};
use test_rust::*;
#[allow(non_snake_case)]

pub fn create_feature_compliance() {
    let connection = &mut establish_connection();

    let FeatureID = 19;
    let ReleaseName = "21.9.5";
    let IsCompliant = true;
    let NumberNonCompliantChildren = 10;
    let LastCheckedDate = NaiveDate::from_ymd_opt(2016, 7, 8).unwrap().and_hms_opt(9, 10, 11).unwrap();
    create_vsts_feature_compliance(connection, FeatureID, (ReleaseName).to_string(), IsCompliant, NumberNonCompliantChildren, LastCheckedDate);
    println!("\nSaved draft {}", ReleaseName);
}

fn main() {
    
}