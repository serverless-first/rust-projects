extern crate diesel;
extern crate repository;

use self::diesel::prelude::*;
use self::models::*;
use self::repository::*;

fn main() {
    use repository::schema::posts::dsl::*;

    let connection = establish_connection();
    let results = posts
        .filter(published.eq(true))
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading Posts");
    println!("Displaying {} posts", results.len());

    for post in results {
        println!("{}", post.title);
        println!("----------------\n");
        println!("{}", post.body);
    }
}
