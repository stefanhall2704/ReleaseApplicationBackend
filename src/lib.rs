use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;


pub mod models;
pub mod schema;


pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}


use self::models::{NewRelease, Release};

pub fn create_release(conn: &mut PgConnection, Name: String, ReleaseDate: NaiveDateTime, IsOffCycle: Option<bool>, ReleaseStatusID: Option<i32>, DowntimeNotes: Option<String>, ReleaseCommitDate: String, RegressionQueryLink: Option<String>, Description: Option<String>, ChangeControlNumber: Option<String>, TotalWorkItemsTaggedForRelease: Option<i32>, IsReadyForQa: Option<bool>) -> Post {
    use crate::schema::Release;

    let new_release = NewRelease { Name, ReleaseDate, IsOffCycle, ReleaseStatusID, DowntimeNotes, ReleaseCommitDate, RegressionQueryLink, Description, ChangeControlNumber, TotalWorkItemsTaggedForRelease, IsReadyForQa };

    diesel::insert_into(Release::table)
        .values(&new_release)
        .get_result(conn)
        .expect("Error saving new post")
}