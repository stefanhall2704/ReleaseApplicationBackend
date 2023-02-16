#![allow(non_snake_case)]
use chrono::prelude::*;
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

//Team API's
#[post("/api/team", format = "application/json", data = "<json>")]
pub fn create_team(json: Json<JsonValue>) -> Json<JsonValue> {
    let connection = &mut establish_connection();

    let data_string = json.to_string();
    let data: &str = &data_string;
    let v: Value = serde_json::from_str(data).unwrap();

    let name = v["name"].as_str().unwrap().to_owned();
    let is_active: bool = v["is_active"].as_bool().unwrap();
    let source_control_team_id = v["source_control_team_id"].as_str().unwrap().to_owned();

    create_db_team(
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

#[get("/api/team/<id>")]
pub fn get_application_team(id: i32) -> Result<std::string::String, ()> {
    let application_team = get_db_team_by_id(id).unwrap();
    let user_json = to_string(&application_team).unwrap();
    Ok(user_json)
}

#[post("/api/team/<id>", format = "application/json", data = "<json>")]
pub fn update_team_api(id: i32, json: Json<JsonValue>) -> Json<JsonValue> {
    let connection = &mut establish_connection();

    let data_string = json.to_string();
    let data: &str = &data_string;
    let v: Value = serde_json::from_str(data).unwrap();

    let name = v["name"].as_str().unwrap().to_owned();
    let is_active: bool = v["is_active"].as_bool().unwrap();
    let source_control_team_id = v["source_control_team_id"].as_str().unwrap().to_owned();
    update_db_team(
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

#[delete("/api/team/<id>", format = "application/json", data = "<json>")]
pub fn delete_team_api(id: i32, json: Json<JsonValue>) -> Result<std::string::String, ()> {
    //required print statement
    println!("{}", json.to_string());
    let connection = &mut establish_connection();
    delete_db_team(connection, id);
    let response = format!("Team deleted in database by id: {}", id);
    Ok(response)
}

//User API's
#[post("/api/user/create", format = "application/json", data = "<json>")]
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

    create_db_user(
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

#[get("/api/user/<id>")]
pub fn get_user(id: i32) -> Result<std::string::String, ()> {
    let application_user = get_db_user_by_id(id).unwrap();
    let user_json = to_string(&application_user).unwrap();
    Ok(user_json)
}

#[post("/api/user/<id>", format = "application/json", data = "<json>")]
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
    update_db_user(
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

#[delete("/api/user/<id>", format = "application/json", data = "<json>")]
pub fn delete_user(id: i32, json: Json<JsonValue>) -> Result<std::string::String, ()> {
    //Required print statement
    println!("{}", json.to_string());
    let connection = &mut establish_connection();
    delete_db_user(connection, id);
    let response = format!("User deleted in database by id: {}", id);
    Ok(response)
}

#[post("/api/release", format = "application/json", data = "<json>")]
pub fn create_release(json: Json<JsonValue>) -> Json<JsonValue> {
    let connection = &mut establish_connection();

    let data_string = json.to_string();
    let data: &str = &data_string;
    let v: Value = serde_json::from_str(data).unwrap();

    let name = v["name"].as_str().unwrap().to_owned();
    let release_date: NaiveDateTime = Local::now().naive_local();
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

#[get("/api/release/<id>")]
pub fn get_release(id: i32) -> Result<std::string::String, ()> {
    let release = get_db_release_by_id(id).unwrap();
    let user_json = to_string(&release).unwrap();
    Ok(user_json)
}

#[post("/api/release/<id>", format = "application/json", data = "<json>")]
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

#[delete("/api/release/<id>", format = "application/json", data = "<json>")]
pub fn delete_release(id: i32, json: Json<JsonValue>) -> Result<std::string::String, ()> {
    //required print statement
    println!("{}", json.to_string());
    let connection = &mut establish_connection();
    delete_db_release(connection, id);
    delete_all_db_release_related_categories(id);
    let response = format!("Team deleted in database by id: {}", id);
    Ok(response)
}

#[post("/api/release_activity", format = "application/json", data = "<json>")]
pub fn create_release_activity(json: Json<JsonValue>) -> Json<JsonValue> {
    let connection = &mut establish_connection();

    let data_string = json.to_string();
    let data: &str = &data_string;
    let v: Value = serde_json::from_str(data).unwrap();

    let title = v["title"].as_str().unwrap().to_owned();
    let created_date: NaiveDateTime = Local::now().naive_local();
    let back_out_procedures = v["back_out_procedures"].as_str().unwrap().to_owned();
    let justification_and_priority = v["justification_and_priority"].as_str().unwrap().to_owned();
    let production_validation = v["production_validation"].as_str().unwrap().to_owned();
    let risk = v["risk"].as_str().unwrap().to_owned();
    let risk_level = v["risk_level"].as_str().unwrap().to_owned();
    let priority_level = v["priority_level"].as_str().unwrap().to_owned();
    let requires_downtime = v["requires_downtime"].as_str().unwrap().to_owned();
    let requires_performance_testing = v["requires_performance_testing"]
        .as_str()
        .unwrap()
        .to_owned();
    let application_team_id: i32 = v["application_team_id"].as_i64().unwrap() as i32;
    create_db_release_activity(
        connection,
        title,
        Some(created_date),
        Some(back_out_procedures),
        Some(justification_and_priority),
        Some(production_validation),
        Some(risk),
        Some(risk_level),
        Some(priority_level),
        requires_downtime,
        requires_performance_testing,
        Some(application_team_id),
    );

    let response = json!({
        "received": json.into_inner(),
        "message": format!("Team created in database")
    });

    Json(response)
}

#[get("/api/release_activity/<id>")]
pub fn get_release_activity(id: i32) -> Result<std::string::String, ()> {
    let release = get_db_release_activity_by_id(id).unwrap();
    let user_json = to_string(&release).unwrap();
    Ok(user_json)
}

#[post(
    "/api/release_activity/<id>",
    format = "application/json",
    data = "<json>"
)]
pub fn update_release_activity(id: i32, json: Json<JsonValue>) -> Json<JsonValue> {
    let connection = &mut establish_connection();

    let data_string = json.to_string();
    let data: &str = &data_string;
    let v: Value = serde_json::from_str(data).unwrap();

    let title = v["title"].as_str().unwrap().to_owned();
    let release_id: i32 = v["release_id"].as_i64().unwrap() as i32;
    let back_out_procedures = v["back_out_procedures"].as_str().unwrap().to_owned();
    let justification_and_priority = v["justification_and_priority"].as_str().unwrap().to_owned();
    let production_validation = v["production_validation"].as_str().unwrap().to_owned();
    let risk = v["risk"].as_str().unwrap().to_owned();
    let risk_level = v["risk_level"].as_str().unwrap().to_owned();
    let priority_level = v["priority_level"].as_str().unwrap().to_owned();
    let requires_downtime = v["requires_downtime"].as_str().unwrap().to_owned();
    let requires_performance_testing = v["requires_performance_testing"]
        .as_str()
        .unwrap()
        .to_owned();
    let application_team_id: i32 = v["application_team_id"].as_i64().unwrap() as i32;
    update_db_release_activity(
        connection,
        id,
        title,
        Some(back_out_procedures),
        Some(justification_and_priority),
        Some(production_validation),
        Some(risk),
        Some(risk_level),
        Some(priority_level),
        requires_downtime,
        requires_performance_testing,
        Some(application_team_id),
        Some(release_id),
    );

    let response = json!({
        "received": json.into_inner(),
        "message": format!("Team updated in database")
    });

    Json(response)
}

#[delete(
    "/api/release_activity/<id>",
    format = "application/json",
    data = "<json>"
)]
pub fn delete_release_activity(id: i32, json: Json<JsonValue>) -> Result<std::string::String, ()> {
    //required print statement
    println!("{}", json.to_string());
    let connection = &mut establish_connection();
    delete_db_release_activity(connection, id);
    delete_release_activity_approvals_by_release_activity_id(id);
    delete_db_release_activity_related_task_by_release_activity_id(connection, id);
    let response = format!("Team deleted in database by id: {}", id);
    Ok(response)
}

#[post(
    "/api/release_activity_task",
    format = "application/json",
    data = "<json>"
)]
pub fn create_release_activity_task(json: Json<JsonValue>) -> Json<JsonValue> {
    let connection = &mut establish_connection();

    let data_string = json.to_string();
    let data: &str = &data_string;
    let v: Value = serde_json::from_str(data).unwrap();

    let release_activity_id: i32 = v["release_activity_id"].as_i64().unwrap() as i32;
    let title = v["title"].as_str().unwrap().to_owned();
    let stage_category_id: i32 = v["stage_category_id"].as_i64().unwrap() as i32;
    let deployment_instructions = v["deployment_instructions"].as_str().unwrap().to_owned();
    let octopus_project_id: i32 = v["octopus_project_id"].as_i64().unwrap() as i32;
    let target_environment_id: i32 = v["target_environment_id"].as_i64().unwrap() as i32;
    let is_hidden: bool = v["is_hidden"].as_bool().unwrap();
    let stage_status_id: i32 = v["stage_status_id"].as_i64().unwrap() as i32;
    let prod_user_id: i32 = v["prod_user_id"].as_i64().unwrap() as i32;
    let stage_user_id: i32 = v["stage_user_id"].as_i64().unwrap() as i32;
    let prod_status_id: i32 = v["prod_status_id"].as_i64().unwrap() as i32;
    let stage_sort_order: i32 = v["stage_sort_order"].as_i64().unwrap() as i32;
    let prod_sort_order: i32 = v["prod_sort_order"].as_i64().unwrap() as i32;
    let prod_category_id: i32 = v["prod_category_id"].as_i64().unwrap() as i32;
    let canonical_order: i32 = v["canonical_order"].as_i64().unwrap() as i32;
    let last_modified_by: String = v["last_modified_by"].as_str().unwrap().to_owned();
    let last_modified_date_time: NaiveDateTime = Local::now().naive_local();
    let dependent_task_id: i32 = v["dependent_task_id"].as_i64().unwrap() as i32;
    let octopus_project_selected_version: String = v["octopus_project_selected_version"]
        .as_str()
        .unwrap()
        .to_owned();

    create_db_release_activity_task(
        connection,
        release_activity_id,
        Some(title),
        Some(stage_category_id),
        Some(deployment_instructions),
        Some(octopus_project_id),
        Some(target_environment_id),
        Some(is_hidden),
        Some(stage_status_id),
        Some(prod_user_id),
        Some(stage_user_id),
        Some(prod_status_id),
        Some(stage_sort_order),
        Some(prod_sort_order),
        Some(prod_category_id),
        Some(canonical_order),
        Some(last_modified_by),
        Some(last_modified_date_time),
        Some(dependent_task_id),
        Some(octopus_project_selected_version),
    );

    let response = json!({
        "received": json.into_inner(),
        "message": format!("Team created in database")
    });

    Json(response)
}

#[get("/api/release_activity_task/<id>")]
pub fn get_release_activity_task(id: i32) -> Result<std::string::String, ()> {
    let release_activity_task = get_db_release_activity_task_by_id(id).unwrap();
    let user_json = to_string(&release_activity_task).unwrap();
    Ok(user_json)
}

#[post(
    "/api/update_release_activity_task/<id>",
    format = "application/json",
    data = "<json>"
)]
pub fn update_release_activity_task(id: i32, json: Json<JsonValue>) -> Json<JsonValue> {
    let connection = &mut establish_connection();

    let data_string = json.to_string();
    let data: &str = &data_string;
    let v: Value = serde_json::from_str(data).unwrap();

    let title = v["title"].as_str().unwrap().to_owned();
    let stage_category_id: i32 = v["stage_category_id"].as_i64().unwrap() as i32;
    let deployment_instructions = v["deployment_instructions"].as_str().unwrap().to_owned();
    let octopus_project_id: i32 = v["octopus_project_id"].as_i64().unwrap() as i32;
    let target_environment_id: i32 = v["target_environment_id"].as_i64().unwrap() as i32;
    let is_hidden: bool = v["is_hidden"].as_bool().unwrap();
    let stage_status_id: i32 = v["stage_status_id"].as_i64().unwrap() as i32;
    let prod_user_id: i32 = v["prod_user_id"].as_i64().unwrap() as i32;
    let stage_user_id: i32 = v["stage_user_id"].as_i64().unwrap() as i32;
    let prod_status_id: i32 = v["prod_status_id"].as_i64().unwrap() as i32;
    let stage_sort_order: i32 = v["stage_sort_order"].as_i64().unwrap() as i32;
    let prod_sort_order: i32 = v["prod_sort_order"].as_i64().unwrap() as i32;
    let prod_category_id: i32 = v["prod_category_id"].as_i64().unwrap() as i32;
    let canonical_order: i32 = v["canonical_order"].as_i64().unwrap() as i32;
    let last_modified_by: String = v["last_modified_by"].as_str().unwrap().to_owned();
    let last_modified_date_time: NaiveDateTime = Local::now().naive_local();
    let dependent_task_id: i32 = v["dependent_task_id"].as_i64().unwrap() as i32;
    let octopus_project_selected_version: String = v["octopus_project_selected_version"]
        .as_str()
        .unwrap()
        .to_owned();

    update_db_release_activity_task(
        connection,
        id,
        Some(title),
        Some(stage_category_id),
        Some(deployment_instructions),
        Some(octopus_project_id),
        Some(target_environment_id),
        Some(is_hidden),
        Some(stage_status_id),
        Some(prod_user_id),
        Some(stage_user_id),
        Some(prod_status_id),
        Some(stage_sort_order),
        Some(prod_sort_order),
        Some(prod_category_id),
        Some(canonical_order),
        Some(last_modified_by),
        Some(last_modified_date_time),
        Some(dependent_task_id),
        Some(octopus_project_selected_version),
    );

    let response = json!({
        "received": json.into_inner(),
        "message": format!("Team updated in database")
    });

    Json(response)
}

#[delete(
    "/api/release_activity_task/<id>",
    format = "application/json",
    data = "<json>"
)]
pub fn delete_release_activity_task(
    id: i32,
    json: Json<JsonValue>,
) -> Result<std::string::String, ()> {
    //required print statement
    println!("{}", json.to_string());
    let connection = &mut establish_connection();
    delete_db_release_activity_task(connection, id);
    delete_db_release_activity_related_task_by_task_id(connection, id);
    let response = format!("Team deleted in database by id: {}", id);
    Ok(response)
}

#[post(
    "/api/release_activity_approval/<id>",
    format = "application/json",
    data = "<json>"
)]
pub fn create_release_activity_approval(id: i32, json: Json<JsonValue>) -> Json<JsonValue> {
    let connection = &mut establish_connection();

    let data_string = json.to_string();
    let data: &str = &data_string;
    let v: Value = serde_json::from_str(data).unwrap();

    let release_activity_approval = v["release_activity_approval"].as_str().unwrap();
    let risk_assessment: String = v["risk_assessment"].as_str().unwrap().to_owned();
    let application_user_id: i32 = v["application_user_id"].as_i64().unwrap() as i32;
    let created_date: NaiveDateTime = Local::now().naive_local();
    let status: String = v["status"].as_str().unwrap().to_owned();
    let comments: String = v["comments"].as_str().unwrap().to_owned();

    let release_approval_type_id = match_release_activity_approval_type(release_activity_approval);
    create_db_release_activity_approval(
        connection,
        id,
        Some(release_approval_type_id),
        Some(risk_assessment),
        Some(application_user_id),
        Some(created_date),
        Some(status),
        Some(comments),
    );

    let risk_assessment: String = v["risk_assessment"].as_str().unwrap().to_owned();
    let application_user_id: i32 = v["application_user_id"].as_i64().unwrap() as i32;
    let created_date: NaiveDateTime = Local::now().naive_local();
    let status: String = v["status"].as_str().unwrap().to_owned();
    let comments: String = v["comments"].as_str().unwrap().to_owned();
    determine_release_approval(
        connection,
        id,
        Some(risk_assessment),
        Some(application_user_id),
        Some(created_date),
        Some(status),
        Some(comments),
    );

    let response = json!({
        "received": json.into_inner(),
        "message": format!("Team created in database")
    });

    Json(response)
}

#[delete(
    "/api/release_activity_approval",
    format = "application/json",
    data = "<json>"
)]
pub fn delete_release_activity_approval(json: Json<JsonValue>) -> Json<JsonValue> {
    println!("{}", json.to_string());
    let connection = &mut establish_connection();

    let data_string = json.to_string();
    let data: &str = &data_string;
    let v: Value = serde_json::from_str(data).unwrap();

    let release_activity_id: i32 = v["release_activity_id"].as_i64().unwrap() as i32;
    let release_activity_approval = v["release_activity_approval"].as_str().unwrap();

    let release_approval_type_id = match_release_activity_approval_type(release_activity_approval);

    delete_db_release_activity_approval(connection, release_approval_type_id, release_activity_id);

    let response = json!({
        "received": json.into_inner(),
        "message": format!("Team created in database")
    });

    Json(response)
}

#[post("/api/category", format = "application/json", data = "<json>")]
pub fn create_release_related_category(json: Json<JsonValue>) -> Json<JsonValue> {
    let connection = &mut establish_connection();

    let data_string = json.to_string();
    let data: &str = &data_string;
    let v: Value = serde_json::from_str(data).unwrap();

    let category = v["category"].as_str().unwrap().to_owned();
    let release_id: i32 = v["release_id"].as_i64().unwrap() as i32;
    let sort_order: i32 = v["sort_order"].as_i64().unwrap() as i32;

    create_db_release_related_category(connection, Some(category), release_id, sort_order);

    let response = json!({
        "received": json.into_inner(),
        "message": format!("Team created in database")
    });

    Json(response)
}

#[delete("/api/category/<id>", format = "application/json", data = "<json>")]
pub fn delete_release_related_category(id: i32, json: Json<JsonValue>) -> Json<JsonValue> {
    println!("{}", json.to_string());

    delete_db_release_related_category(id);

    let response = json!({
        "received": json.into_inner(),
        "message": format!("Team created in database")
    });

    Json(response)
}
