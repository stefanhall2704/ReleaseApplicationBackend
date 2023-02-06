#![feature(proc_macro_hygiene, decl_macro)]
#![allow(non_snake_case)]
use rocket::{routes};
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
extern crate serde_json;
use self::api::*;
pub mod api;

fn main() {
    rocket::ignite()
        .mount("/", routes![delete_team_api, update_team_api, get_feature_compliance, create_feature_compliance, create_team, get_application_team])
        .launch();
}