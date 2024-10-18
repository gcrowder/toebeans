use diesel::prelude::*;
use crate::schema::animal;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::animal)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Animal {
    pub id: i32,
    pub name: String,
    pub species: Option<String>,
    pub microchip: Option<i32>
}

#[derive(Insertable)]
#[diesel(table_name = animal)]
pub struct NewAnimal {
    pub name: String,
    pub species: Option<String>,
    pub microchip: Option<i32>
}