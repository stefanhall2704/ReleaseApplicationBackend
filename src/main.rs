#![feature(proc_macro_hygiene, decl_macro)]
#![allow(non_snake_case)]
use rocket::routes;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
extern crate serde_json;
use self::api::*;
pub mod api;

fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![
                download_file,
                read_file_bytes,
                get_release_release_related_categories,
                delete_release_related_category,
                create_release_related_category,
                delete_release_activity_approval,
                create_release_activity_approval,
                update_release_activity_task,
                delete_release_activity_task,
                get_release_activity_task,
                create_release_activity_task,
                delete_release_activity,
                update_release_activity,
                get_release_activity,
                create_release_activity,
                create_release,
                update_release,
                delete_release,
                get_release,
                delete_user,
                update_user,
                get_user,
                create_user,
                delete_team_api,
                update_team_api,
                create_team,
                get_application_team
            ],
        )
        .launch();
}
