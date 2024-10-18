use toebeans::models::animal::Animal;
use diesel::prelude::*;
use toebeans::*;

fn main() {
    use crate::schema::animal;

    let connection = &mut establish_connection();
    let results = animal::table
        .limit(5)
        .select(Animal::as_select())
        .load(connection)
        .expect("Error loading animals");

    println!("Displaying {} animals", results.len());
    for animal in results {
        println!("{}", animal.name);
        println!("-----------\n");
        println!("{}", animal.species.unwrap_or("Not known".to_string()));
    }
}