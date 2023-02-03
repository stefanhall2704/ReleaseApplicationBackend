#![feature(proc_macro_hygiene, decl_macro)]
// pub mod write_vsts_feature_compliance;
#![allow(non_snake_case)]
use chrono::NaiveDate;
use std::result::Result;

use rocket::{routes, post, get};
// use serde::{Serialize, Deserialize};
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
extern crate serde_json;
use serde_json::{Value};
use test_rust::*;
use rocket_contrib::json::{Json, JsonValue};
use rocket_contrib::json;

use rocket::http::Status;


#[get("/api/getFeatureComplianceById/<id>")]
fn get_feature_compliance(id: i32) -> Result<String, Status> {
    get_feature_compliance_by_id(id);
    Ok("Hello, world!".to_owned())
    // println!("This worked, id: {}", id);
}


#[post("/api/createFeatureCompliance", format = "application/json", data = "<json>")]
fn create_feature_compliance(json: Json<JsonValue>) -> Json<JsonValue> {
    let connection = &mut establish_connection();

    let data_string = json.to_string();
    let data: &str = &data_string;
    let v: Value = serde_json::from_str(data).unwrap();

    let feature_id: i32 = v["feature_id"].as_i64().unwrap() as i32;
    let release_name = v["release_name"].as_str().unwrap().to_owned();
    let is_compliant = v["is_compliant"].is_boolean();

    let number_non_compliant_children: i32 = v["number_non_compliant_children"].as_i64().unwrap() as i32;

    let last_checked_date = NaiveDate::from_ymd_opt(2016, 7, 8).unwrap().and_hms_opt(9, 10, 11).unwrap();
    create_vsts_feature_compliance(connection, feature_id, (release_name).to_string(), is_compliant, number_non_compliant_children, last_checked_date);

    let response = json!({
        "received": json.into_inner(),
        "message": format!("Release added to Database: {release_name}")
    });

    Json(response)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![create_feature_compliance, get_feature_compliance])
        .launch();
}
