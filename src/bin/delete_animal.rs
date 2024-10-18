use diesel::prelude::*;
use toebeans::*;
use std::env::args;
use toebeans::schema::animal::id;

fn main() {
    use self::schema::animal::dsl::animal as animal_schema;

    let target_id = args().nth(1).expect("Expected a target to match against").parse::<i32>().expect("Invalid ID");

    let connection = &mut establish_connection();
    let num_deleted = diesel::delete(animal_schema.filter(id.eq(target_id)))
        .execute(connection)
        .expect("Error deleting animal");

    println!("Deleted {} animals", num_deleted);
}