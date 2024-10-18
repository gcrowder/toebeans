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
use models::{NewAnimal, Animal};

pub fn create_animal(conn: &mut PgConnection, name: String, species: Option<String>, microchip: Option<i32>) -> Animal {
    use crate::schema::animal;
    let new_animal = NewAnimal { name, species, microchip };

    diesel::insert_into(animal::table)
        .values(&new_animal)
        .returning(Animal::as_returning())
        .get_result(conn)
        .expect("Error saving new animal")
}