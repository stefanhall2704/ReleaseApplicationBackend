// use test_rust::*;
// use diesel::prelude::*;
// use serde::Serialize;
// #[allow(non_snake_case)]

// pub fn select_compliance_feature(id: i32) {
//     use self::schema::VstsFeatureCompliance::dsl::*;
//     use self::models::VstsFeatureCompliance as featureCompliance;
//     use schema::VstsFeatureCompliance::dsl::{ID as Feature_id, *};
//     let connection = &mut establish_connection();

//     let result = VstsFeatureCompliance
//         .filter(Feature_id.eq(id))
//         .select((Feature_id, FeatureID, ReleaseName, IsCompliant, NumberNonCompliantChildren, LastCheckedDate))
//         .first::<featureCompliance>(connection);
//         Ok(serde_json::to_string(&result)?)
// }

fn main() {

}