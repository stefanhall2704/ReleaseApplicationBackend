use self::models::*;
use diesel::prelude::*;
use diesel_demo::*;

fn main() {
    use self::schema::Release::dsl::*;

    let connection = &mut establish_connection();
    let results = Release
        .filter(published.eq(true))
        .limit(5)
        .load::<Post>(connection)
        .expect("Error loading posts");

    println!("Displaying {} Releases", results.len());
    for post in results {
        println!("{}", post.title);
        println!("-----------\n");
        println!("{}", post.body);
    }
}