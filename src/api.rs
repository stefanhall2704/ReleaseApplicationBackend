#![allow(non_snake_case)]
use chrono::NaiveDate;
use rocket::{delete, get, post};
use std::result::Result;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
extern crate serde_json;
use rocket_contrib::json;
use rocket_contrib::json::{Json, JsonValue};
use serde_json::{to_string, Value};
use test_rust::*;

#[get("/api/getFeatureComplianceById/<id>")]
pub fn get_feature_compliance(id: i32) -> Result<std::string::String, ()> {
    let feature_compliance = get_feature_compliance_by_id(id).unwrap();
    let user_json = to_string(&feature_compliance).unwrap();
    Ok(user_json)
}

#[post(
    "/api/createFeatureCompliance",
    format = "application/json",
    data = "<json>"
)]
pub fn create_feature_compliance(json: Json<JsonValue>) -> Json<JsonValue> {
    let connection = &mut establish_connection();

    let data_string = json.to_string();
    let data: &str = &data_string;
    let v: Value = serde_json::from_str(data).unwrap();

    let feature_id: i32 = v["feature_id"].as_i64().unwrap() as i32;
    let release_name = v["release_name"].as_str().unwrap().to_owned();
    let is_compliant = v["is_compliant"].is_boolean();

    let number_non_compliant_children: i32 =
        v["number_non_compliant_children"].as_i64().unwrap() as i32;

    let last_checked_date = NaiveDate::from_ymd_opt(2016, 7, 8)
        .unwrap()
        .and_hms_opt(9, 10, 11)
        .unwrap();
    create_vsts_feature_compliance(
        connection,
        feature_id,
        (release_name).to_string(),
        is_compliant,
        number_non_compliant_children,
        last_checked_date,
    );

    let response = json!({
        "received": json.into_inner(),
        "message": format!("Release added to Database: {release_name}")
    });

    Json(response)
}

//Team API's
#[post("/api/createTeam", format = "application/json", data = "<json>")]
pub fn create_team(json: Json<JsonValue>) -> Json<JsonValue> {
    let connection = &mut establish_connection();

    let data_string = json.to_string();
    let data: &str = &data_string;
    let v: Value = serde_json::from_str(data).unwrap();

    let name = v["name"].as_str().unwrap().to_owned();
    let is_active: bool = v["is_active"].as_bool().unwrap();
    let source_control_team_id = v["source_control_team_id"].as_str().unwrap().to_owned();

    create_application_team(
        connection,
        name,
        Some(is_active),
        Some(source_control_team_id),
    );

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

#[post("/api/updateTeam/<id>", format = "application/json", data = "<json>")]
pub fn update_team_api(id: i32, json: Json<JsonValue>) -> Json<JsonValue> {
    let connection = &mut establish_connection();

    let data_string = json.to_string();
    let data: &str = &data_string;
    let v: Value = serde_json::from_str(data).unwrap();

    let name = v["name"].as_str().unwrap().to_owned();
    let is_active: bool = v["is_active"].as_bool().unwrap();
    let source_control_team_id = v["source_control_team_id"].as_str().unwrap().to_owned();
    update_application_team(
        connection,
        id,
        name,
        Some(is_active),
        Some(source_control_team_id),
    );

    let response = json!({
        "received": json.into_inner(),
        "message": format!("Team updated in database")
    });

    Json(response)
}

#[delete("/api/deleteTeam/<id>", format = "application/json", data = "<json>")]
pub fn delete_team_api(id: i32, json: Json<JsonValue>) -> Result<std::string::String, ()> {
    //required print statement
    println!("{}", json.to_string());
    let connection = &mut establish_connection();
    delete_application_team(connection, id);
    let response = format!("Team deleted in database by id: {}", id);
    Ok(response)
}

//User API's
#[post("/api/createUser", format = "application/json", data = "<json>")]
pub fn create_user(json: Json<JsonValue>) -> Json<JsonValue> {
    let connection = &mut establish_connection();

    let data_string = json.to_string();
    let data: &str = &data_string;
    let v: Value = serde_json::from_str(data).unwrap();

    let first = v["first"].as_str().unwrap().to_owned();
    let last = v["last"].as_str().unwrap().to_owned();
    let ad_username = v["ad_username"].as_str().unwrap().to_owned();
    let email = v["email"].as_str().unwrap().to_owned();
    let primary_phone = v["primary_phone"].as_str().unwrap().to_owned();
    let secondary_phone = v["secondary_phone"].as_str().unwrap().to_owned();
    let application_user_role_id: i32 = v["application_user_role_id"].as_i64().unwrap() as i32;
    let is_active: i32 = v["application_team_id"].as_i64().unwrap() as i32;
    let application_team_id: i32 = v["is_active"].as_i64().unwrap() as i32;

    create_application_user(
        connection,
        first,
        last,
        ad_username,
        email,
        Some(primary_phone),
        Some(secondary_phone),
        application_user_role_id,
        Some(is_active),
        Some(application_team_id),
    );

    let response = json!({
        "received": json.into_inner(),
        "message": format!("User created in database")
    });

    Json(response)
}

#[get("/api/getUser/<id>")]
pub fn get_application_user(id: i32) -> Result<std::string::String, ()> {
    let application_user = get_user_team_by_id(id).unwrap();
    let user_json = to_string(&application_user).unwrap();
    Ok(user_json)
}

#[post("/api/updateUser/<id>", format = "application/json", data = "<json>")]
pub fn update_user(id: i32, json: Json<JsonValue>) -> Json<JsonValue> {
    let connection = &mut establish_connection();

    let data_string = json.to_string();
    let data: &str = &data_string;
    let v: Value = serde_json::from_str(data).unwrap();

    let first = v["first"].as_str().unwrap().to_owned();
    let last = v["last"].as_str().unwrap().to_owned();
    let ad_username = v["ad_username"].as_str().unwrap().to_owned();
    let email = v["email"].as_str().unwrap().to_owned();
    let primary_phone = v["primary_phone"].as_str().unwrap().to_owned();
    let secondary_phone = v["secondary_phone"].as_str().unwrap().to_owned();
    let application_user_role_id: i32 = v["application_user_role_id"].as_i64().unwrap() as i32;
    let is_active: i32 = v["application_team_id"].as_i64().unwrap() as i32;
    let application_team_id: i32 = v["is_active"].as_i64().unwrap() as i32;
    let release_approval_type_ids = v["release_approval_type_ids"].as_array().unwrap();

    add_release_approval_type(connection, id, release_approval_type_ids);
    update_application_user(
        connection,
        id,
        first,
        last,
        ad_username,
        email,
        Some(primary_phone),
        Some(secondary_phone),
        application_user_role_id,
        Some(is_active),
        Some(application_team_id),
    );

    let response = json!({
        "received": json.into_inner(),
        "message": format!("User updated in database")
    });

    Json(response)
}

#[delete("/api/deleteUser/<id>", format = "application/json", data = "<json>")]
pub fn delete_user(id: i32, json: Json<JsonValue>) -> Result<std::string::String, ()> {
    //Required print statement
    println!("{}", json.to_string());
    let connection = &mut establish_connection();
    delete_application_user(connection, id);
    let response = format!("User deleted in database by id: {}", id);
    Ok(response)
}

#[post("/api/createRelease", format = "application/json", data = "<json>")]
pub fn create_release(json: Json<JsonValue>) -> Json<JsonValue> {
    let connection = &mut establish_connection();

    let data_string = json.to_string();
    let data: &str = &data_string;
    let v: Value = serde_json::from_str(data).unwrap();

    let name = v["name"].as_str().unwrap().to_owned();
    let release_date = NaiveDate::from_ymd_opt(2023, 2, 5)
        .unwrap()
        .and_hms_opt(9, 10, 11)
        .unwrap();
    let is_off_cycle = v["is_off_cycle"].as_bool().unwrap();
    let release_status_id: i32 = v["release_status_id"].as_i64().unwrap() as i32;
    let downtime_notes = v["downtime_notes"].as_str().unwrap().to_owned();
    let release_commit_date = v["release_commit_date"].as_str().unwrap().to_owned();
    let regression_query_link = v["regression_query_link"].as_str().unwrap().to_owned();
    let description = v["description"].as_str().unwrap().to_owned();
    let change_control_number = v["change_control_number"].as_str().unwrap().to_owned();
    let total_work_items_tagged_for_release: i32 =
        v["total_work_items_tagged_for_release"].as_i64().unwrap() as i32;
    let is_ready_for_qa = v["is_ready_for_qa"].as_bool().unwrap();

    create_db_release(
        connection,
        name,
        release_date,
        Some(is_off_cycle),
        Some(release_status_id),
        Some(downtime_notes),
        release_commit_date,
        Some(regression_query_link),
        Some(description),
        Some(change_control_number),
        Some(total_work_items_tagged_for_release),
        Some(is_ready_for_qa),
    );

    let response = json!({
        "received": json.into_inner(),
        "message": format!("Team created in database")
    });

    Json(response)
}

#[get("/api/getRelease/<id>")]
pub fn get_release(id: i32) -> Result<std::string::String, ()> {
    let release = get_db_release_by_id(id).unwrap();
    let user_json = to_string(&release).unwrap();
    Ok(user_json)
}

#[post(
    "/api/updateRelease/<id>",
    format = "application/json",
    data = "<json>"
)]
pub fn update_release(id: i32, json: Json<JsonValue>) -> Json<JsonValue> {
    let connection = &mut establish_connection();

    let data_string = json.to_string();
    let data: &str = &data_string;
    let v: Value = serde_json::from_str(data).unwrap();

    let name = v["name"].as_str().unwrap().to_owned();
    let release_date = NaiveDate::from_ymd_opt(2023, 2, 5)
        .unwrap()
        .and_hms_opt(9, 10, 11)
        .unwrap();
    let is_off_cycle = v["is_off_cycle"].as_bool().unwrap();
    let release_status_id: i32 = v["release_status_id"].as_i64().unwrap() as i32;
    let downtime_notes = v["downtime_notes"].as_str().unwrap().to_owned();
    let release_commit_date = v["release_commit_date"].as_str().unwrap().to_owned();
    let regression_query_link = v["regression_query_link"].as_str().unwrap().to_owned();
    let description = v["description"].as_str().unwrap().to_owned();
    let change_control_number = v["change_control_number"].as_str().unwrap().to_owned();
    let total_work_items_tagged_for_release: i32 =
        v["total_work_items_tagged_for_release"].as_i64().unwrap() as i32;
    let is_ready_for_qa = v["is_ready_for_qa"].as_bool().unwrap();

    update_db_release(
        connection,
        id,
        name,
        release_date,
        Some(is_off_cycle),
        Some(release_status_id),
        Some(downtime_notes),
        release_commit_date,
        Some(regression_query_link),
        Some(description),
        Some(change_control_number),
        Some(total_work_items_tagged_for_release),
        Some(is_ready_for_qa),
    );

    let response = json!({
        "received": json.into_inner(),
        "message": format!("Team updated in database")
    });

    Json(response)
}

#[delete(
    "/api/deleteRelease/<id>",
    format = "application/json",
    data = "<json>"
)]
pub fn delete_release(id: i32, json: Json<JsonValue>) -> Result<std::string::String, ()> {
    //required print statement
    println!("{}", json.to_string());
    let connection = &mut establish_connection();
    delete_db_release(connection, id);
    let response = format!("Team deleted in database by id: {}", id);
    Ok(response)
}
