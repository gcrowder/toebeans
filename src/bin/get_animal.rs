use models::Animal;
use diesel::prelude::*;
use toebeans::*;
use std::env::args;

fn main() {
    use self::schema::animal::dsl::animal as animal_schema;

    let animal_id = args()
        .nth(1)
        .expect("get_animal requires an animal id")
        .parse::<i32>()
        .expect("Invalid ID");

    let connection = &mut establish_connection();

    let animal = animal_schema
        .find(animal_id)
        .select(Animal::as_select())
        .first(connection)
        .optional(); // This allows for returning an Option<Animal>, otherwise it will throw an error

    match animal {
        Ok(Some(animal)) => println!("Animal with id: {} has Name: {} and Species: {}", animal.id, animal.name, animal.species.unwrap_or("unknown".to_string())),
        Ok(None) => println!("Unable to find animal {}", animal_id),
        Err(_) => println!("An error occured while fetching animal {}", animal_id),
    }
}