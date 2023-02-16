use chrono::prelude::*;
use chrono::NaiveDateTime;
use diesel::connection::Connection;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenvy::dotenv;
use serde_json::Value;
use std::collections::HashSet;
use std::env;

use self::models::NewReleaseRelatedCategory;
use self::models::ReleaseRelatedCategory as release_related_category;
use self::schema::ReleaseRelatedCategory as release_related_category_schema;

use self::models::NewReleaseActivityApproval;
use self::models::ReleaseActivityApproval as release_activity_approval;
use self::schema::ReleaseActivityApproval as release_activity_approval_schema;

use self::models::NewReleaseActivityRelatedTask;
use self::models::ReleaseActivityRelatedTask as release_activity_related_task;
use self::schema::ReleaseActivityRelatedTask as release_activity_related_task_schema;

use self::models::NewReleaseActivityTask;
use self::models::ReleaseActivityTask as release_activity_task;
use self::schema::ReleaseActivityTask as release_activity_task_schema;

use self::models::NewReleaseActivity;
use self::models::ReleaseActivity as release_activity;
use self::models::UpdateReleaseActivity;
use self::schema::ReleaseActivity as release_activity_schema;

use self::models::NewRelease;
use self::models::Release as release;
use self::schema::Release as release_schema;

use self::models::ApplicationUserReleaseApproval as application_user_release_approval;
use self::models::NewApplicationUserReleaseApproval;
use self::schema::ApplicationUserReleaseApproval as application_user_release_approval_schema;

use self::models::ApplicationUser as application_user;
use self::models::NewApplicationUser;
use self::schema::ApplicationUser as application_user_schema;

use self::models::ApplicationTeam as application_team;
use self::models::NewApplicationTeam;
use self::schema::ApplicationTeam as application_team_schema;

pub mod models;
pub mod schema;

pub enum ReleaseActivityApprovalTypes {
    Testing = 1,
    Regression = 2,
    Performance = 3,
    Release = 4,
}

pub fn match_release_activity_approval_type(approval_match: &str) -> i32 {
    let approval: i32;
    match approval_match {
        "Testing" => approval = ReleaseActivityApprovalTypes::Testing as i32,
        "Regression" => approval = ReleaseActivityApprovalTypes::Regression as i32,
        "Performance" => approval = ReleaseActivityApprovalTypes::Performance as i32,
        "Release" => approval = ReleaseActivityApprovalTypes::Release as i32,
        _ => approval = -1,
    }
    approval
}

#[allow(non_snake_case)]
pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

//Team Database Functions
pub fn create_db_team(
    conn: &mut SqliteConnection,
    name: String,
    is_active: Option<bool>,
    source_control_team_id: Option<String>,
) {
    let application_team = NewApplicationTeam {
        Name: name,
        IsActive: is_active,
        SourceControlTeamID: source_control_team_id,
    };

    diesel::insert_into(application_team_schema::table)
        .values(&application_team)
        .execute(conn)
        .expect("Error saving new team");
}

pub fn get_db_team_by_id(id: i32) -> Result<NewApplicationTeam, ()> {
    let connection = &mut establish_connection();

    let application_team_db = application_team_schema::table
        .filter(application_team_schema::ID.eq(id))
        .first::<application_team>(connection)
        .unwrap();

    let data = NewApplicationTeam {
        Name: application_team_db.Name,
        IsActive: application_team_db.IsActive,
        SourceControlTeamID: application_team_db.SourceControlTeamID,
    };
    Ok(data)
}

pub fn update_db_team(
    conn: &mut SqliteConnection,
    id: i32,
    name: String,
    is_active: Option<bool>,
    source_control_team_id: Option<String>,
) {
    let application_team_db = NewApplicationTeam {
        Name: name,
        IsActive: is_active,
        SourceControlTeamID: source_control_team_id,
    };

    diesel::update(application_team_schema::table.find(id))
        .set((
            application_team_schema::Name.eq(application_team_db.Name),
            application_team_schema::IsActive.eq(application_team_db.IsActive),
            application_team_schema::SourceControlTeamID
                .eq(application_team_db.SourceControlTeamID),
        ))
        .execute(conn)
        .expect("Error updating team");
}

pub fn delete_db_team(conn: &mut SqliteConnection, id: i32) {
    diesel::delete(application_team_schema::table.find(id))
        .execute(conn)
        .expect("Error deleting team");
}

pub fn get_user_release_approval_ids_by_user_id(
    id: i32,
) -> Result<Vec<NewApplicationUserReleaseApproval>, ()> {
    let connection = &mut establish_connection();

    let release_approval_ids = application_user_release_approval_schema::table
        .filter(application_user_release_approval_schema::ApplicationUserID.eq(id))
        .load::<application_user_release_approval>(connection)
        .unwrap();
    let mut release_approval_type_ids_vec = Vec::new();
    for release_approval_id in release_approval_ids {
        let data = NewApplicationUserReleaseApproval {
            ApplicationUserID: release_approval_id.ApplicationUserID,
            ReleaseApprovalTypeID: release_approval_id.ReleaseApprovalTypeID,
        };
        release_approval_type_ids_vec.push(data);
    }

    Ok(release_approval_type_ids_vec)
}

fn add_db_user_release_approval_type_id(application_user_id: i32, release_approval_type_id: i32) {
    let conn = &mut establish_connection();
    let application_user_approval_type = NewApplicationUserReleaseApproval {
        ApplicationUserID: application_user_id,
        ReleaseApprovalTypeID: release_approval_type_id,
    };

    diesel::insert_into(application_user_release_approval_schema::table)
        .values(&application_user_approval_type)
        .execute(conn)
        .expect("Error saving new user");
}

fn delete_db_user_release_approval_type_id(
    conn: &mut SqliteConnection,
    user_id: i32,
    release_approval_type_id: i32,
) {
    diesel::delete(
        application_user_release_approval_schema::table.filter(
            application_user_release_approval_schema::ApplicationUserID
                .eq(user_id)
                .and(
                    application_user_release_approval_schema::ReleaseApprovalTypeID
                        .eq(release_approval_type_id),
                ),
        ),
    )
    .execute(conn)
    .expect("Error deleting approval id");
}

fn delete_unrequired_user_release_approval_type_ids<'a>(
    conn: &mut SqliteConnection,
    user_id: i32,
    release_approval_type_ids_db: Vec<&'a i32>,
    release_approval_type_ids: Vec<i32>,
) {
    let mut approval_ids_to_delete = vec![];
    for approval_id in release_approval_type_ids_db {
        if !release_approval_type_ids.contains(&approval_id) {
            approval_ids_to_delete.push(approval_id);
        }
    }
    for release_approval_id in approval_ids_to_delete {
        delete_db_user_release_approval_type_id(conn, user_id, *release_approval_id);
    }
}

fn convert_value_to_int(release_approval_type_id: &Value) -> i32 {
    match release_approval_type_id {
        &serde_json::Value::Number(ref n) => {
            if let Some(i) = n.as_i64() {
                let approval_type_id = i as i32;
                approval_type_id
            } else {
                0
            }
        }
        _ => 0,
    }
}

fn get_needed_user_release_approval_type_ids<'a>(
    conn: &'a mut SqliteConnection,
    application_user_id: i32,
    release_approval_type_id: &'a i32,
    all_release_approval_type_ids: Vec<i32>,
) -> Result<Vec<&'a i32>, ()> {
    let release_approval_type_ids_db_one =
        get_user_release_approval_ids_by_user_id(application_user_id);
    let mut release_approval_type_ids = Vec::new();
    let mut vec_of_needed_release_approval_type_ids = Vec::new();
    for release_approval_type_ids_db in release_approval_type_ids_db_one.iter() {
        for release_approval_type_id_db in release_approval_type_ids_db.iter() {
            let approval_id = release_approval_type_id_db.ReleaseApprovalTypeID;
            release_approval_type_ids.push(approval_id);
        }
        let unique: HashSet<i32> = release_approval_type_ids.drain(..).collect();
        let v_unique: Vec<i32> = unique.into_iter().collect();
        let final_convert: Vec<&i32> = v_unique.iter().map(|x| x).collect();
        delete_unrequired_user_release_approval_type_ids(
            conn,
            application_user_id,
            final_convert,
            all_release_approval_type_ids.clone(),
        );
        if v_unique.contains(release_approval_type_id) {
            //Required print Statement
            println!("Value Exists");
        } else {
            vec_of_needed_release_approval_type_ids.push(release_approval_type_id);
        }
    }
    Ok(vec_of_needed_release_approval_type_ids)
}

fn convert_json_value_to_vec_of_ints(vec: &Vec<serde_json::Value>) -> Vec<i32> {
    vec.iter()
        .filter_map(|value| value.as_i64().map(|v| v as i32))
        .collect()
}

fn add_new_user_release_approval_type_ids(
    conn: &mut SqliteConnection,
    application_user_id: i32,
    approval_type_id: i32,
    requested_release_approval_type_ids: &Vec<Value>,
) {
    if approval_type_id != 0 {
        let release_approval_type_ids =
            convert_json_value_to_vec_of_ints(&requested_release_approval_type_ids);
        let new_release_approval_type_ids = get_needed_user_release_approval_type_ids(
            conn,
            application_user_id,
            &approval_type_id,
            release_approval_type_ids,
        );

        for approval_ids_db in new_release_approval_type_ids.iter() {
            for approval_id_db in approval_ids_db.iter() {
                add_db_user_release_approval_type_id(application_user_id, **approval_id_db);
            }
        }
    } else {
        //Required print Statement
        println!("Value is 0");
    }
}

pub fn add_release_approval_type(
    conn: &mut SqliteConnection,
    application_user_id: i32,
    release_approval_type_ids: &Vec<Value>,
) {
    let requested_release_approval_type_ids = release_approval_type_ids.clone();
    for release_approval_type_id in release_approval_type_ids {
        let approval_type_id: i32 = convert_value_to_int(release_approval_type_id);
        add_new_user_release_approval_type_ids(
            conn,
            application_user_id,
            approval_type_id,
            &requested_release_approval_type_ids,
        );
    }
}

//User Database Functions
pub fn create_db_user(
    conn: &mut SqliteConnection,
    first: String,
    last: String,
    ad_username: String,
    email: String,
    primary_phone: Option<String>,
    secondary_phone: Option<String>,
    application_user_role_id: i32,
    is_active: Option<i32>,
    application_team_id: Option<i32>,
) {
    let application_user_db = NewApplicationUser {
        First: first,
        Last: last,
        ADUsername: ad_username,
        Email: email,
        PrimaryPhone: primary_phone,
        SecondaryPhone: secondary_phone,
        ApplicationUserRoleID: application_user_role_id,
        IsActive: is_active,
        ApplicationTeamID: application_team_id,
    };

    diesel::insert_into(application_user_schema::table)
        .values(&application_user_db)
        .execute(conn)
        .expect("Error saving new user");
}

pub fn get_db_user_by_id(id: i32) -> Result<NewApplicationUser, ()> {
    let connection = &mut establish_connection();

    let application_team_db = application_user_schema::table
        .filter(application_user_schema::ID.eq(id))
        .first::<application_user>(connection)
        .unwrap();
    let data = NewApplicationUser {
        First: application_team_db.First,
        Last: application_team_db.Last,
        ADUsername: application_team_db.ADUsername,
        Email: application_team_db.Email,
        PrimaryPhone: application_team_db.PrimaryPhone,
        SecondaryPhone: application_team_db.SecondaryPhone,
        ApplicationUserRoleID: application_team_db.ApplicationUserRoleID,
        IsActive: application_team_db.IsActive,
        ApplicationTeamID: application_team_db.ApplicationTeamID,
    };
    Ok(data)
}

pub fn update_db_user(
    conn: &mut SqliteConnection,
    id: i32,
    first: String,
    last: String,
    ad_username: String,
    email: String,
    primary_phone: Option<String>,
    secondary_phone: Option<String>,
    application_user_role_id: i32,
    is_active: Option<i32>,
    application_team_id: Option<i32>,
) {
    let application_user_db = NewApplicationUser {
        First: first,
        Last: last,
        ADUsername: ad_username,
        Email: email,
        PrimaryPhone: primary_phone,
        SecondaryPhone: secondary_phone,
        ApplicationUserRoleID: application_user_role_id,
        IsActive: is_active,
        ApplicationTeamID: application_team_id,
    };

    diesel::update(application_user_schema::table.find(id))
        .set((
            application_user_schema::First.eq(application_user_db.First),
            application_user_schema::Last.eq(application_user_db.Last),
            application_user_schema::ADUsername.eq(application_user_db.ADUsername),
            application_user_schema::Email.eq(application_user_db.Email),
            application_user_schema::PrimaryPhone.eq(application_user_db.PrimaryPhone),
            application_user_schema::SecondaryPhone.eq(application_user_db.SecondaryPhone),
            application_user_schema::ApplicationUserRoleID
                .eq(application_user_db.ApplicationUserRoleID),
            application_user_schema::IsActive.eq(application_user_db.IsActive),
            application_user_schema::ApplicationTeamID.eq(application_user_db.ApplicationTeamID),
        ))
        .execute(conn)
        .expect("Error updating post");
}

pub fn delete_db_user(conn: &mut SqliteConnection, id: i32) {
    diesel::delete(application_user_schema::table.find(id))
        .execute(conn)
        .expect("Error deleting user");
}

//Releases
pub fn create_db_release(
    conn: &mut SqliteConnection,
    name: String,
    release_date: NaiveDateTime,
    is_off_cycle: Option<bool>,
    release_status_id: Option<i32>,
    downtime_notes: Option<String>,
    release_commit_date: String,
    regression_query_link: Option<String>,
    description: Option<String>,
    change_control_number: Option<String>,
    total_work_items_tagged_for_release: Option<i32>,
    is_ready_for_qa: Option<bool>,
) {
    let release_db = NewRelease {
        Name: name,
        ReleaseDate: release_date,
        IsOffCycle: is_off_cycle,
        ReleaseStatusID: release_status_id,
        DowntimeNotes: downtime_notes,
        ReleaseCommitDate: release_commit_date,
        RegressionQueryLink: regression_query_link,
        Description: description,
        ChangeControlNumber: change_control_number,
        TotalWorkItemsTaggedForRelease: total_work_items_tagged_for_release,
        IsReadyForQa: is_ready_for_qa,
    };

    diesel::insert_into(release_schema::table)
        .values(&release_db)
        .execute(conn)
        .expect("Error saving new release");
}

pub fn get_db_release_by_id(id: i32) -> Result<NewRelease, ()> {
    let connection = &mut establish_connection();

    let release_db = release_schema::table
        .filter(release_schema::ID.eq(id))
        .first::<release>(connection)
        .unwrap();

    let data = NewRelease {
        Name: release_db.Name,
        ReleaseDate: release_db.ReleaseDate,
        IsOffCycle: release_db.IsOffCycle,
        ReleaseStatusID: release_db.ReleaseStatusID,
        DowntimeNotes: release_db.DowntimeNotes,
        ReleaseCommitDate: release_db.ReleaseCommitDate,
        RegressionQueryLink: release_db.RegressionQueryLink,
        Description: release_db.Description,
        ChangeControlNumber: release_db.ChangeControlNumber,
        TotalWorkItemsTaggedForRelease: release_db.TotalWorkItemsTaggedForRelease,
        IsReadyForQa: release_db.IsReadyForQa,
    };
    Ok(data)
}

pub fn update_db_release(
    conn: &mut SqliteConnection,
    id: i32,
    name: String,
    release_date: NaiveDateTime,
    is_off_cycle: Option<bool>,
    release_status_id: Option<i32>,
    downtime_notes: Option<String>,
    release_commit_date: String,
    regression_query_link: Option<String>,
    description: Option<String>,
    change_control_number: Option<String>,
    total_work_items_tagged_for_release: Option<i32>,
    is_ready_for_qa: Option<bool>,
) {
    let release_db = NewRelease {
        Name: name,
        ReleaseDate: release_date,
        IsOffCycle: is_off_cycle,
        ReleaseStatusID: release_status_id,
        DowntimeNotes: downtime_notes,
        ReleaseCommitDate: release_commit_date,
        RegressionQueryLink: regression_query_link,
        Description: description,
        ChangeControlNumber: change_control_number,
        TotalWorkItemsTaggedForRelease: total_work_items_tagged_for_release,
        IsReadyForQa: is_ready_for_qa,
    };

    diesel::update(release_schema::table.find(id))
        .set((
            release_schema::Name.eq(release_db.Name),
            release_schema::ReleaseDate.eq(release_db.ReleaseDate),
            release_schema::IsOffCycle.eq(release_db.IsOffCycle),
            release_schema::ReleaseStatusID.eq(release_db.ReleaseStatusID),
            release_schema::DowntimeNotes.eq(release_db.DowntimeNotes),
            release_schema::ReleaseCommitDate.eq(release_db.ReleaseCommitDate),
            release_schema::RegressionQueryLink.eq(release_db.RegressionQueryLink),
            release_schema::Description.eq(release_db.Description),
            release_schema::ChangeControlNumber.eq(release_db.ChangeControlNumber),
            release_schema::TotalWorkItemsTaggedForRelease
                .eq(release_db.TotalWorkItemsTaggedForRelease),
            release_schema::IsReadyForQa.eq(release_db.IsReadyForQa),
        ))
        .execute(conn)
        .expect("Error updating release");
}

pub fn delete_db_release(conn: &mut SqliteConnection, id: i32) {
    diesel::delete(release_schema::table.find(id))
        .execute(conn)
        .expect("Error deleting release");
}

pub fn create_db_release_activity(
    conn: &mut SqliteConnection,
    title: String,
    created_date: Option<NaiveDateTime>,
    back_out_procedures: Option<String>,
    justification_and_priority: Option<String>,
    production_validation: Option<String>,
    risk: Option<String>,
    risk_level: Option<String>,
    priority_level: Option<String>,
    requires_downtime: String,
    requires_performance_testing: String,
    application_team_id: Option<i32>,
) {
    let last_modified_date: NaiveDateTime = Local::now().naive_local();
    let release_activity_db = NewReleaseActivity {
        Title: title,
        CreatedDate: created_date,
        BackOutProcedures: back_out_procedures,
        JustificationAndPriority: justification_and_priority,
        ProductionValidation: production_validation,
        Risk: risk,
        RiskLevel: risk_level,
        PriorityLevel: priority_level,
        RequiresDowntime: requires_downtime,
        RequiresPerformanceTesting: requires_performance_testing,
        ApplicationTeamID: application_team_id,
        LastModifiedDate: Some(last_modified_date),
    };

    diesel::insert_into(release_activity_schema::table)
        .values(&release_activity_db)
        .execute(conn)
        .expect("Error saving new release activity");
}

pub fn get_db_release_activity_by_id(id: i32) -> Result<NewReleaseActivity, ()> {
    let connection = &mut establish_connection();

    let release_activity_db = release_activity_schema::table
        .filter(release_activity_schema::ID.eq(id))
        .first::<release_activity>(connection)
        .unwrap();

    let data = NewReleaseActivity {
        Title: release_activity_db.Title,
        CreatedDate: release_activity_db.CreatedDate,
        BackOutProcedures: release_activity_db.BackOutProcedures,
        JustificationAndPriority: release_activity_db.JustificationAndPriority,
        ProductionValidation: release_activity_db.ProductionValidation,
        Risk: release_activity_db.Risk,
        RiskLevel: release_activity_db.RiskLevel,
        PriorityLevel: release_activity_db.PriorityLevel,
        RequiresDowntime: release_activity_db.RequiresDowntime,
        RequiresPerformanceTesting: release_activity_db.RequiresPerformanceTesting,
        ApplicationTeamID: release_activity_db.ApplicationTeamID,
        LastModifiedDate: release_activity_db.LastModifiedDate,
    };
    Ok(data)
}

pub fn update_db_release_activity(
    conn: &mut SqliteConnection,
    id: i32,
    title: String,
    back_out_procedures: Option<String>,
    justification_and_priority: Option<String>,
    production_validation: Option<String>,
    risk: Option<String>,
    risk_level: Option<String>,
    priority_level: Option<String>,
    requires_downtime: String,
    requires_performance_testing: String,
    application_team_id: Option<i32>,
    release_id: Option<i32>,
) {
    let last_modified_date: NaiveDateTime = Local::now().naive_local();

    let release_activity_db = UpdateReleaseActivity {
        Title: title,
        ReleaseID: release_id,
        BackOutProcedures: back_out_procedures,
        JustificationAndPriority: justification_and_priority,
        ProductionValidation: production_validation,
        Risk: risk,
        RiskLevel: risk_level,
        PriorityLevel: priority_level,
        RequiresDowntime: requires_downtime,
        RequiresPerformanceTesting: requires_performance_testing,
        ApplicationTeamID: application_team_id,
        LastModifiedDate: Some(last_modified_date),
    };

    diesel::update(release_activity_schema::table.find(id))
        .set((
            release_activity_schema::Title.eq(release_activity_db.Title),
            release_activity_schema::ReleaseID.eq(release_activity_db.ReleaseID),
            release_activity_schema::BackOutProcedures.eq(release_activity_db.BackOutProcedures),
            release_activity_schema::JustificationAndPriority
                .eq(release_activity_db.JustificationAndPriority),
            release_activity_schema::ProductionValidation
                .eq(release_activity_db.ProductionValidation),
            release_activity_schema::Risk.eq(release_activity_db.Risk),
            release_activity_schema::RiskLevel.eq(release_activity_db.RiskLevel),
            release_activity_schema::PriorityLevel.eq(release_activity_db.PriorityLevel),
            release_activity_schema::RequiresDowntime.eq(release_activity_db.RequiresDowntime),
            release_activity_schema::RequiresPerformanceTesting
                .eq(release_activity_db.RequiresPerformanceTesting),
            release_activity_schema::ApplicationTeamID.eq(release_activity_db.ApplicationTeamID),
            release_activity_schema::LastModifiedDate.eq(last_modified_date),
        ))
        .execute(conn)
        .expect("Error updating release activity");
}

pub fn delete_db_release_activity(conn: &mut SqliteConnection, id: i32) {
    diesel::delete(release_activity_schema::table.find(id))
        .execute(conn)
        .expect("Error deleting release activity");
}

pub fn create_db_release_activity_related_task(
    conn: &mut SqliteConnection,
    release_activity_id: i32,
    release_activity_task_id: i32,
    octopus_project_selected_version: Option<String>,
) {
    let release_activity_related_task = NewReleaseActivityRelatedTask {
        ReleaseActivityID: release_activity_id,
        ReleaseActivityTaskID: release_activity_task_id,
        OctopusProjectSelectedVersion: octopus_project_selected_version,
    };
    diesel::insert_into(release_activity_related_task_schema::table)
        .values(&release_activity_related_task)
        .execute(conn)
        .expect("Error saving new release activity related task");
}

pub fn get_db_release_activity_task_by_title(
    title: Option<String>,
) -> Result<release_activity_task, ()> {
    let connection = &mut establish_connection();

    let release_activity_task = release_activity_task_schema::table
        .filter(release_activity_task_schema::Title.eq(title))
        .first::<release_activity_task>(connection)
        .unwrap();
    Ok(release_activity_task)
}

pub fn create_db_release_activity_task(
    conn: &mut SqliteConnection,
    release_activity_id: i32,
    title: Option<String>,
    stage_category_id: Option<i32>,
    deployment_instructions: Option<String>,
    octopus_project_id: Option<i32>,
    target_environment_id: Option<i32>,
    is_hidden: Option<bool>,
    stage_status_id: Option<i32>,
    prod_user_id: Option<i32>,
    stage_user_id: Option<i32>,
    prod_status_id: Option<i32>,
    stage_sort_order: Option<i32>,
    prod_sort_order: Option<i32>,
    prod_category_id: Option<i32>,
    canonical_order: Option<i32>,
    last_modified_by: Option<String>,
    last_modified_date_time: Option<NaiveDateTime>,
    dependent_task_id: Option<i32>,
    octopus_project_selected_version: Option<String>,
) {
    let title_clone = title.clone();
    let clone_octopus_project_selected_version = octopus_project_selected_version.clone();
    let release_activity_task_db = NewReleaseActivityTask {
        Title: title,
        StageCategoryID: stage_category_id,
        DeploymentInstructions: deployment_instructions,
        OctopusProjectID: octopus_project_id,
        TargetEnvironmentID: target_environment_id,
        IsHidden: is_hidden,
        StageStatusID: stage_status_id,
        ProdUserID: prod_user_id,
        StageUserID: stage_user_id,
        ProdStatusID: prod_status_id,
        StageSortOrder: stage_sort_order,
        ProdSortOrder: prod_sort_order,
        ProdCategoryID: prod_category_id,
        CanonicalOrder: canonical_order,
        LastModifiedBy: last_modified_by,
        LastModifiedDateTime: last_modified_date_time,
        DependentTaskID: dependent_task_id,
        OctopusProjectSelectedVersion: octopus_project_selected_version,
    };

    diesel::insert_into(release_activity_task_schema::table)
        .values(&release_activity_task_db)
        .execute(conn)
        .expect("Error saving new release activity task");

    let release_activity_task_by_title =
        get_db_release_activity_task_by_title(title_clone).unwrap();
    let release_activity_task_id = release_activity_task_by_title.ID;
    println!("{}", release_activity_task_id.to_string());

    create_db_release_activity_related_task(
        conn,
        release_activity_id,
        release_activity_task_id,
        clone_octopus_project_selected_version,
    );
}

pub fn get_release_activity_tasks_by_release_activity_id(
    release_activity_id: i32,
) -> Result<Vec<models::ReleaseActivityRelatedTask>, ()> {
    let connection = &mut establish_connection();

    let release_activity_related_tasks = release_activity_related_task_schema::table
        .filter(release_activity_related_task_schema::ReleaseActivityID.eq(release_activity_id))
        .load::<release_activity_related_task>(connection)
        .unwrap();
    Ok(release_activity_related_tasks)
}

pub fn get_db_release_activity_task_by_id(id: i32) -> Result<NewReleaseActivityTask, ()> {
    let connection = &mut establish_connection();

    let release_activity_task = release_activity_task_schema::table
        .filter(release_activity_task_schema::ID.eq(id))
        .first::<release_activity_task>(connection)
        .unwrap();
    let release_activity_task_db = NewReleaseActivityTask {
        Title: release_activity_task.Title,
        StageCategoryID: release_activity_task.StageCategoryID,
        DeploymentInstructions: release_activity_task.DeploymentInstructions,
        OctopusProjectID: release_activity_task.OctopusProjectID,
        TargetEnvironmentID: release_activity_task.TargetEnvironmentID,
        IsHidden: release_activity_task.IsHidden,
        StageStatusID: release_activity_task.StageStatusID,
        ProdUserID: release_activity_task.ProdUserID,
        StageUserID: release_activity_task.StageUserID,
        ProdStatusID: release_activity_task.ProdStatusID,
        StageSortOrder: release_activity_task.StageSortOrder,
        ProdSortOrder: release_activity_task.ProdSortOrder,
        ProdCategoryID: release_activity_task.ProdCategoryID,
        CanonicalOrder: release_activity_task.CanonicalOrder,
        LastModifiedBy: release_activity_task.LastModifiedBy,
        LastModifiedDateTime: release_activity_task.LastModifiedDateTime,
        DependentTaskID: release_activity_task.DependentTaskID,
        OctopusProjectSelectedVersion: release_activity_task.OctopusProjectSelectedVersion,
    };

    Ok(release_activity_task_db)
}

pub fn update_db_release_activity_task(
    conn: &mut SqliteConnection,
    id: i32,
    title: Option<String>,
    stage_category_id: Option<i32>,
    deployment_instructions: Option<String>,
    octopus_project_id: Option<i32>,
    target_environment_id: Option<i32>,
    is_hidden: Option<bool>,
    stage_status_id: Option<i32>,
    prod_user_id: Option<i32>,
    stage_user_id: Option<i32>,
    prod_status_id: Option<i32>,
    stage_sort_order: Option<i32>,
    prod_sort_order: Option<i32>,
    prod_category_id: Option<i32>,
    canonical_order: Option<i32>,
    last_modified_by: Option<String>,
    last_modified_date_time: Option<NaiveDateTime>,
    dependent_task_id: Option<i32>,
    octopus_project_selected_version: Option<String>,
) {
    let release_activity_task_db = NewReleaseActivityTask {
        Title: title,
        StageCategoryID: stage_category_id,
        DeploymentInstructions: deployment_instructions,
        OctopusProjectID: octopus_project_id,
        TargetEnvironmentID: target_environment_id,
        IsHidden: is_hidden,
        StageStatusID: stage_status_id,
        ProdUserID: prod_user_id,
        StageUserID: stage_user_id,
        ProdStatusID: prod_status_id,
        StageSortOrder: stage_sort_order,
        ProdSortOrder: prod_sort_order,
        ProdCategoryID: prod_category_id,
        CanonicalOrder: canonical_order,
        LastModifiedBy: last_modified_by,
        LastModifiedDateTime: last_modified_date_time,
        DependentTaskID: dependent_task_id,
        OctopusProjectSelectedVersion: octopus_project_selected_version,
    };

    diesel::update(release_activity_task_schema::table.find(id))
        .set((
            release_activity_task_schema::Title.eq(release_activity_task_db.Title),
            release_activity_task_schema::StageCategoryID
                .eq(release_activity_task_db.StageCategoryID),
            release_activity_task_schema::DeploymentInstructions
                .eq(release_activity_task_db.DeploymentInstructions),
            release_activity_task_schema::OctopusProjectID
                .eq(release_activity_task_db.OctopusProjectID),
            release_activity_task_schema::TargetEnvironmentID
                .eq(release_activity_task_db.TargetEnvironmentID),
            release_activity_task_schema::IsHidden.eq(release_activity_task_db.IsHidden),
            release_activity_task_schema::StageStatusID.eq(release_activity_task_db.StageStatusID),
            release_activity_task_schema::ProdUserID.eq(release_activity_task_db.ProdUserID),
            release_activity_task_schema::StageUserID.eq(release_activity_task_db.StageUserID),
            release_activity_task_schema::ProdStatusID.eq(release_activity_task_db.ProdStatusID),
            release_activity_task_schema::StageSortOrder
                .eq(release_activity_task_db.StageSortOrder),
            release_activity_task_schema::ProdSortOrder.eq(release_activity_task_db.ProdSortOrder),
            release_activity_task_schema::ProdCategoryID
                .eq(release_activity_task_db.ProdCategoryID),
            release_activity_task_schema::CanonicalOrder
                .eq(release_activity_task_db.CanonicalOrder),
            release_activity_task_schema::LastModifiedBy
                .eq(release_activity_task_db.LastModifiedBy),
            release_activity_task_schema::LastModifiedDateTime
                .eq(release_activity_task_db.LastModifiedDateTime),
            release_activity_task_schema::DependentTaskID
                .eq(release_activity_task_db.DependentTaskID),
            release_activity_task_schema::OctopusProjectSelectedVersion
                .eq(release_activity_task_db.OctopusProjectSelectedVersion),
        ))
        .execute(conn)
        .expect("Error updating release activity task");
}

pub fn delete_db_release_activity_task(conn: &mut SqliteConnection, id: i32) {
    diesel::delete(release_activity_task_schema::table.find(id))
        .execute(conn)
        .expect("Error deleting release activity task");
}

pub fn delete_db_release_activity_related_task_by_task_id(conn: &mut SqliteConnection, id: i32) {
    diesel::delete(
        release_activity_related_task_schema::table
            .filter(release_activity_related_task_schema::ReleaseActivityTaskID.eq(id)),
    )
    .execute(conn)
    .expect("Error deleting release activity related task");
}

fn delete_release_activity_tasks_by_activity_id(
    conn: &mut SqliteConnection,
    release_activity_id: i32,
) {
    let release_activity_tasks =
        get_release_activity_tasks_by_release_activity_id(release_activity_id);
    if let Ok(release_activity_task) = release_activity_tasks {
        for task in release_activity_task {
            let release_activity_task_id = task.ID;
            delete_db_release_activity_task(conn, release_activity_task_id);
        }
    }
}

pub fn delete_db_release_activity_related_task_by_release_activity_id(
    conn: &mut SqliteConnection,
    release_activity_id: i32,
) {
    delete_release_activity_tasks_by_activity_id(conn, release_activity_id);
    diesel::delete(
        release_activity_related_task_schema::table.filter(
            release_activity_related_task_schema::ReleaseActivityID.eq(release_activity_id),
        ),
    )
    .execute(conn)
    .expect("Error deleting release activity related task by release activity id");
}

pub fn determine_release_approval(
    conn: &mut SqliteConnection,
    release_activity_id: i32,
    risk_assessment: Option<String>,
    application_user_id: Option<i32>,
    created_date: Option<NaiveDateTime>,
    status: Option<String>,
    comments: Option<String>,
) {
    let required_approvals_for_release_auto_approval = vec![1, 2, 3];
    let mut existing_approvals_in_release_activity: Vec<i32> = Vec::new();
    let release_activity_approvals =
        get_release_activity_approvals_by_release_activity_id(release_activity_id);
    if let Ok(approvals) = release_activity_approvals {
        for approval in approvals {
            let approval_id: Option<i32> = approval.ReleaseApprovalTypeID;
            match approval_id {
                Some(value) => {
                    existing_approvals_in_release_activity.push(value);
                }
                None => {
                    println!("The value is absent");
                }
            }
        }
    }
    existing_approvals_in_release_activity.sort();
    if existing_approvals_in_release_activity == required_approvals_for_release_auto_approval {
        let release_approval = match_release_activity_approval_type("Release");
        create_db_release_activity_approval(
            conn,
            release_activity_id,
            Some(release_approval),
            risk_assessment,
            application_user_id,
            created_date,
            status,
            comments,
        );
    }
}

pub fn get_release_activity_approvals_by_release_activity_id(
    release_activity_id: i32,
) -> Result<Vec<models::ReleaseActivityApproval>, ()> {
    let connection = &mut establish_connection();

    let release_activity_approval = release_activity_approval_schema::table
        .filter(release_activity_approval_schema::ReleaseActivityID.eq(release_activity_id))
        .load::<release_activity_approval>(connection)
        .unwrap();
    Ok(release_activity_approval)
}

pub fn delete_release_activity_approvals_by_release_activity_id(release_activity_id: i32) {
    let release_activity_approvals =
        get_release_activity_approvals_by_release_activity_id(release_activity_id);
    let connection = &mut establish_connection();
    if let Ok(release_activity_approval) = release_activity_approvals {
        for approval in release_activity_approval {
            let approval_id: Option<i32> = approval.ReleaseApprovalTypeID;
            match approval_id {
                Some(value) => {
                    delete_db_release_activity_approval(connection, value, release_activity_id);
                    println!("The value is {}", value);
                }
                None => {
                    println!("The value is absent");
                }
            }
        }
    }
}

pub fn create_db_release_activity_approval(
    conn: &mut SqliteConnection,
    release_activity_id: i32,
    release_approval_type_id: Option<i32>,
    risk_assessment: Option<String>,
    application_user_id: Option<i32>,
    created_date: Option<NaiveDateTime>,
    status: Option<String>,
    comments: Option<String>,
) {
    let release_activity_approval_db = NewReleaseActivityApproval {
        ReleaseActivityID: release_activity_id,
        ReleaseApprovalTypeID: release_approval_type_id,
        RiskAssessment: risk_assessment,
        ApplicationUserID: application_user_id,
        CreatedDate: created_date,
        Status: status,
        Comments: comments,
    };

    diesel::insert_into(release_activity_approval_schema::table)
        .values(&release_activity_approval_db)
        .execute(conn)
        .expect("Error saving new release activity approval");
}

pub fn delete_db_release_activity_approval(
    conn: &mut SqliteConnection,
    approval_type_id: i32,
    release_activity_id: i32,
) {
    diesel::delete(
        release_activity_approval_schema::table.filter(
            release_activity_approval_schema::ReleaseActivityID
                .eq(release_activity_id)
                .and(release_activity_approval_schema::ReleaseApprovalTypeID.eq(approval_type_id)),
        ),
    )
    .execute(conn)
    .expect("Error deleting release activity approval");
}

pub fn get_db_release_categories(
    conn: &mut SqliteConnection,
    release_id: i32,
) -> Result<Vec<models::ReleaseRelatedCategory>, ()> {
    let release_related_category_db = release_related_category_schema::table
        .filter(release_related_category_schema::ReleaseID.eq(release_id))
        .load::<release_related_category>(conn)
        .unwrap();
    Ok(release_related_category_db)
}

pub fn get_db_release_related_category_by_id(id: i32) -> Result<NewReleaseRelatedCategory, ()> {
    let connection = &mut establish_connection();

    let release_related_category_db = release_related_category_schema::table
        .filter(release_related_category_schema::ID.eq(id))
        .first::<release_related_category>(connection)
        .unwrap();

    let data = NewReleaseRelatedCategory {
        Category: release_related_category_db.Category,
        ReleaseID: release_related_category_db.ReleaseID,
        SortOrder: release_related_category_db.SortOrder,
    };
    Ok(data)
}

pub fn get_next_category_sort_order(release_id: i32) -> Result<i32, ()> {
    let connection = &mut establish_connection();
    let release_related_categories = get_db_release_categories(connection, release_id).unwrap();
    let count_of_release_related_categories: i32 =
        release_related_categories.len().try_into().unwrap();
    let new_count = count_of_release_related_categories + 1;
    Ok(new_count)
}

pub fn create_db_release_related_category(
    conn: &mut SqliteConnection,
    category: Option<String>,
    release_id: i32,
    sort_order: i32,
) {
    let sort_order_number: i32;
    let new_sort_order = get_next_category_sort_order(release_id).unwrap();
    if new_sort_order >= 1 {
        sort_order_number = new_sort_order;
    } else {
        sort_order_number = sort_order
    }
    let data = NewReleaseRelatedCategory {
        Category: category,
        ReleaseID: release_id,
        SortOrder: sort_order_number,
    };
    diesel::insert_into(release_related_category_schema::table)
        .values(&data)
        .execute(conn)
        .expect("Error saving new release related category");
}

pub fn delete_db_release_related_category(id: i32) {
    let conn = &mut establish_connection();
    diesel::delete(release_related_category_schema::table.find(id))
        .execute(conn)
        .expect("Error deleting release related category");
}

pub fn delete_all_db_release_related_categories(release_id: i32) {
    let conn = &mut establish_connection();
    diesel::delete(
        release_related_category_schema::table
            .filter(release_related_category_schema::ReleaseID.eq(release_id)),
    )
    .execute(conn)
    .expect("Error deleting release related categories by release id");
}
