#![allow(non_snake_case)]
use chrono::NaiveDate;
use std::result::Result;
use rocket::{post, get};
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
extern crate serde_json;
use serde_json::{Value, to_string};
use test_rust::*;
use rocket_contrib::json::{Json, JsonValue};
use rocket_contrib::json;


#[get("/api/getFeatureComplianceById/<id>")]
pub fn get_feature_compliance(id: i32) -> Result<std::string::String, ()> {
    let feature_compliance = get_feature_compliance_by_id(id).unwrap();
    let user_json = to_string(&feature_compliance).unwrap();
    Ok(user_json)
}


#[post("/api/createFeatureCompliance", format = "application/json", data = "<json>")]
pub fn create_feature_compliance(json: Json<JsonValue>) -> Json<JsonValue> {
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


#[post("/api/createTeam", format = "application/json", data = "<json>")]
pub fn create_team(json: Json<JsonValue>) -> Json<JsonValue> {
    let connection = &mut establish_connection();

    let data_string = json.to_string();
    let data: &str = &data_string;
    let v: Value = serde_json::from_str(data).unwrap();

    let name= v["name"].as_str().unwrap().to_owned();
    let is_active: bool = v["is_active"].is_boolean();
    let source_control_team_id = v["source_control_team_id"].as_str().unwrap().to_owned();

    create_application_team(connection, name, is_active, source_control_team_id);

    let response = json!({
        "received": json.into_inner(),
        "message": format!("Team created in database")
    });

    Json(response)
}


#[get("/api/getTeam/<id>")]
pub fn get_application_team(id: i32) -> Result<std::string::String, ()> {
    let application_team = get_application_team_by_id(id).unwrap();
    let user_json = to_string(&application_team).unwrap();
    Ok(user_json)
}