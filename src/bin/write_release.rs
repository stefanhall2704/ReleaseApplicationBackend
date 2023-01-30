use diesel_demo::*;
use std::io::{stdin, Read};

fn main() {
    let connection = &mut establish_connection();

    let mut Name = String::new();
    let mut ReleaseDate = NaiveDateTime::new();
    let mut IsOffCycle = Option<bool>::new();
    let mut ReleaseStatusID = Option<i32>::new();
    let mut DowntimeNotes = Option<String>::new();
    let mut ReleaseCommitDate = String::new();
    let mut RegressionQueryLink = Option<String>::new();
    let mut Description = Option<String>::new();
    let mut ChangeControlNumber = Option<String>::new();
    let mut TotalWorkItemsTaggedForRelease = Option<i32>::new();
    let mut IsReadyForQa = Option<bool>::new();


    stdin().read_line(&mut Name).unwrap();
    let Name = Name.trim_end();

    stdin().read_line(&mut ReleaseDate).unwrap();
    let ReleaseDate = ReleaseDate.trim_end();

    stdin().read_line(&mut IsOffCycle).unwrap();
    let IsOffCycle = IsOffCycle.trim_end();

    stdin().read_line(&mut ReleaseStatusID).unwrap();
    let ReleaseStatusID = ReleaseStatusID.trim_end();

    stdin().read_line(&mut ReleaseStatusID).unwrap();
    let ReleaseStatusID = ReleaseStatusID.trim_end();

    stdin().read_line(&mut DowntimeNotes).unwrap();
    let DowntimeNotes = DowntimeNotes.trim_end();

    stdin().read_line(&mut ReleaseCommitDate).unwrap();
    let ReleaseCommitDate = ReleaseCommitDate.trim_end();

    stdin().read_line(&mut RegressionQueryLink).unwrap();
    let RegressionQueryLink = RegressionQueryLink.trim_end();

    stdin().read_line(&mut Description).unwrap();
    let Description = Description.trim_end();

    stdin().read_line(&mut ChangeControlNumber).unwrap();
    let ChangeControlNumber = ChangeControlNumber.trim_end();

    stdin().read_line(&mut TotalWorkItemsTaggedForRelease).unwrap();
    let TotalWorkItemsTaggedForRelease = TotalWorkItemsTaggedForRelease.trim_end();

    stdin().read_line(&mut IsReadyForQa).unwrap();
    let IsReadyForQa = IsReadyForQa.trim_end();

    println!(
        "\nOk! Let's write {} (Press {} when finished)\n",
        title, EOF
    );
    stdin().read_to_string(&mut body).unwrap();

    let post = create_post(connection, Name, ReleaseDate, IsOffCycle, ReleaseStatusID, DowntimeNotes, ReleaseCommitDate, RegressionQueryLink, Description, ChangeControlNumber, TotalWorkItemsTaggedForRelease, IsReadyForQa);

    println!("\nSaved draft {} with id {}", connection, Name, ReleaseDate, IsOffCycle, ReleaseStatusID, DowntimeNotes, ReleaseCommitDate, RegressionQueryLink, Description, ChangeControlNumber, TotalWorkItemsTaggedForRelease, IsReadyForQa);
}

#[cfg(not(windows))]
const EOF: &str = "CTRL+D";

#[cfg(windows)]
const EOF: &str = "CTRL+Z";