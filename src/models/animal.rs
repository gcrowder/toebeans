use std::str::FromStr;
use diesel::AsExpression;
use diesel::backend::Backend;
use diesel::prelude::*;
use diesel::deserialize::FromSql;
use diesel::serialize::{Output, ToSql, self};
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

#[derive(Clone, Debug, Serialize, Deserialize, AsExpression)]
pub enum Species {
    Cat,
    Dog,
    Bird,

}

impl Species {
    fn as_str(&self) -> &'static str {
        match self {
            Species::Cat => "cat",
            Species::Dog => "dog",
            Species::Bird => "bird",
        }
    }
}
impl FromStr for Species {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cat" => Ok(Species::Cat),
            "dog" => Ok(Species::Dog),
            "bird" => Ok(Species::Bird),
            _ => Err(())
        }
    }
}
impl FromSql<diesel::sql_types::Text, diesel::pg::Pg> for Species {
    fn from_sql(bytes: diesel::pg::PgValue<'_>) -> diesel::deserialize::Result<Self> {
        let s = <String as FromSql<diesel::sql_types::Text, diesel::pg::Pg>>::from_sql(bytes)?;
        Species::from_str(&s).map_err(|_| "Invalid Species".into())
    }
}

impl<DB> ToSql<String, DB> for Species
where
    DB: Backend,
    String: ToSql<String, DB>,
{
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, DB>) -> serialize::Result {
        self.to_string().to_sql(out)
    }
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
    pub async fn create(incoming_animal: NewAnimal) -> Result<Animal, diesel::result::Error> {
        let connection = &mut establish_connection();
        let saved_animal = incoming_animal.insert_into(animal::table).get_result(connection);
        Ok(saved_animal.unwrap())
    }
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = animal)]
pub struct NewAnimal {
    pub name: String,
    pub species: Option<Species>,
    pub microchip: Option<i32>
}