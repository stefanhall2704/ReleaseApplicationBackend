use chrono::NaiveDateTime;
use diesel::connection::Connection;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenvy::dotenv;
use std::collections::HashSet;
use std::env;

use self::models::ApplicationUserReleaseApproval as application_user_release_approval;
use self::models::NewApplicationUserReleaseApproval;
use self::schema::ApplicationUserReleaseApproval as application_user_release_approval_schema;

use self::models::ApplicationUser as application_user;
use self::models::NewApplicationUser;
use self::schema::ApplicationUser as application_user_schema;

use self::models::ApplicationTeam as application_team;
use self::models::NewApplicationTeam;
use self::schema::ApplicationTeam as application_team_schema;

use self::models::NewVstsFeatureCompliance;
use self::models::VstsFeatureCompliance as featureCompliance;
use self::schema::VstsFeatureCompliance as featureComplianceSchema;
use serde_json::Value;


pub mod models;
pub mod schema;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[allow(non_snake_case)]
pub fn create_vsts_feature_compliance(
    conn: &mut SqliteConnection,
    feature_id: i32,
    release_name: String,
    is_compliant: bool,
    compliant_children: i32,
    last_checked_date: NaiveDateTime,
) {
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
        .first::<featureCompliance>(connection)
        .unwrap();
    let data = NewVstsFeatureCompliance {
        FeatureID: feature_compliance.FeatureID,
        ReleaseName: feature_compliance.ReleaseName,
        IsCompliant: feature_compliance.IsCompliant,
        NumberNonCompliantChildren: feature_compliance.NumberNonCompliantChildren,
        LastCheckedDate: feature_compliance.LastCheckedDate,
    };
    Ok(data)
}

//Team Database Functions
#[allow(non_snake_case)]
pub fn create_application_team(
    conn: &mut SqliteConnection,
    name: String,
    is_active: Option<bool>,
    source_control_team_id: Option<String>,
) {
    // let optional_int = is_active.unwrap();
    let application_team = NewApplicationTeam {
        Name: name,
        IsActive: is_active,
        SourceControlTeamID: source_control_team_id,
    };

    diesel::insert_into(application_team_schema::table)
        .values(&application_team)
        .execute(conn)
        .expect("Error saving new post");
}

pub fn get_application_team_by_id(id: i32) -> Result<NewApplicationTeam, ()> {
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

pub fn update_application_team(
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
        .expect("Error saving new post");
}

pub fn delete_application_team(conn: &mut SqliteConnection, id: i32) {
    diesel::delete(application_team_schema::table.find(id))
        .execute(conn)
        .expect("Error saving new post");
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

fn add_application_user_approval_type_id(
    application_user_id: i32,
    release_approval_type_id: i32,
) {
    let conn = &mut establish_connection();
    let application_user_approval_type = NewApplicationUserReleaseApproval {
        ApplicationUserID: application_user_id,
        ReleaseApprovalTypeID: release_approval_type_id,
    };

    diesel::insert_into(application_user_release_approval_schema::table)
        .values(&application_user_approval_type)
        .execute(conn)
        .expect("Error saving new post");
}

pub fn delete_release_approval_type_id(
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
    .expect("Error saving new post");
}


fn detect_duplicate_release_approval_type_ids<'a>(conn: &mut SqliteConnection, user_id: i32, release_approval_type_ids_db: Vec<&'a i32>, release_approval_type_ids: Vec<i32>) {
    let mut approval_ids_to_delete = vec![];
    for approval_id in release_approval_type_ids_db {
        if !release_approval_type_ids.contains(&approval_id) {
            approval_ids_to_delete.push(approval_id);
        }
    }
    for release_approval_id in approval_ids_to_delete {
        delete_release_approval_type_id(conn, user_id, *release_approval_id);
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

fn check_for_existing_application_user_release_approval_type_ids<'a>(
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
        println!("{:#?}", release_approval_type_ids);
        let unique: HashSet<i32> = release_approval_type_ids.drain(..).collect();
        let v_unique: Vec<i32> = unique.into_iter().collect();
        let final_convert: Vec<&i32> = v_unique.iter().map(|x| x).collect();
        detect_duplicate_release_approval_type_ids(conn, application_user_id, final_convert, all_release_approval_type_ids.clone());
        if v_unique.contains(release_approval_type_id) {
            println!("Value Exists");
        } else {
            vec_of_needed_release_approval_type_ids.push(release_approval_type_id);
        }
    }
    Ok(vec_of_needed_release_approval_type_ids)
}

fn convert_vec_type(vec: &Vec<serde_json::Value>) -> Vec<i32>{
    vec.iter()
        .filter_map(|value| value.as_i64().map(|v| v as i32))
        .collect()

}

fn check_for_zero_value(
    conn: &mut SqliteConnection,
    application_user_id: i32,
    approval_type_id: i32,
    requested_release_approval_type_ids: &Vec<Value>,
) {
    if approval_type_id != 0 {
        let final_requested_approval_ids = convert_vec_type(&requested_release_approval_type_ids);
        let new_release_approval_type_ids =
            check_for_existing_application_user_release_approval_type_ids(
                conn,
                application_user_id,
                &approval_type_id,
                final_requested_approval_ids
            );

        // let requested_release_approval_type_ids = new_release_approval_type_ids.copy();
        for approval_ids_db in new_release_approval_type_ids.iter() {
            for approval_id_db in approval_ids_db.iter() {
                add_application_user_approval_type_id(application_user_id, **approval_id_db);
            }
        }
    } else {
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
        check_for_zero_value(conn, application_user_id, approval_type_id, &requested_release_approval_type_ids);
    }
}

//User Database Functions
pub fn create_application_user(
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
        .expect("Error saving new post");
}

pub fn get_user_team_by_id(id: i32) -> Result<NewApplicationUser, ()> {
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

pub fn update_application_user(
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
        .expect("Error saving new post");
}

pub fn delete_application_user(conn: &mut SqliteConnection, id: i32) {
    diesel::delete(application_user_schema::table.find(id))
        .execute(conn)
        .expect("Error saving new post");
}
