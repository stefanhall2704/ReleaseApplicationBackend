use chrono::{NaiveDate};
use test_rust::*;
use diesel::prelude::*;
#[allow(non_snake_case)]

pub fn create_feature_compliance() {
    let connection = &mut establish_connection();

    let FeatureID = 19;
    let ReleaseName = "21.9.5";
    let IsCompliant = true;
    let NumberNonCompliantChildren = 10;
    let LastCheckedDate = NaiveDate::from_ymd_opt(2016, 7, 8).unwrap().and_hms_opt(9, 10, 11).unwrap();
    // create_vsts_feature_compliance(connection, FeatureID, (ReleaseName).to_string(), IsCompliant, NumberNonCompliantChildren, LastCheckedDate);
    println!("\nSaved draft {}", ReleaseName);
}




pub fn select_compliance_feature(id: i32) -> Result<Result<test_rust::models::VstsFeatureCompliance, diesel::result::Error>, ()>{
    use self::schema::VstsFeatureCompliance::dsl::*;
    use self::models::VstsFeatureCompliance as featureCompliance;
    use schema::VstsFeatureCompliance::dsl::{ID as Feature_id, *};
    let connection = &mut establish_connection();

    let result = VstsFeatureCompliance
        .filter(Feature_id.eq(id))
        .select((Feature_id, FeatureID, ReleaseName, IsCompliant, NumberNonCompliantChildren, LastCheckedDate))
        .first::<featureCompliance>(connection);
    // Ok(serde_json::to_string(&result)?)
    Ok(result)
}


fn main() {

}