// @generated automatically by Diesel CLI.

diesel::table! {
    animal (id) {
        id -> Int4,
        name -> Varchar,
        species -> Nullable<Varchar>,
        microchip -> Nullable<Int4>,
    }
}
