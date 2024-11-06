use diesel::prelude::*;
use serde::{ Serialize, Deserialize };
use crate::establish_connection;
use crate::schema::animal;

#[derive(Queryable, Selectable, Debug, Clone, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::animal)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Animal {
    pub id: i32,
    pub name: String,
    pub species: Option<String>,
    pub microchip: Option<i32>
}

impl Animal {
    pub async fn get_animal(animal_id: i32) -> Result<Animal, diesel::result::Error> {
        let connection = &mut establish_connection();
        match animal::dsl::animal
            .find(animal_id)
            .select(Animal::as_select())
            .first(connection) {
                Ok(animal) => Ok(animal),
                Err(_) => Err(diesel::result::Error::NotFound)
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = animal)]
pub struct NewAnimal {
    pub name: String,
    pub species: Option<String>,
    pub microchip: Option<i32>
}