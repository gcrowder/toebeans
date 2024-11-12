use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router, extract::Path
};
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};

use diesel::prelude::*;
use toebeans::establish_connection;
use toebeans::models::{Animal, NewAnimal};
use toebeans::schema::animal::dsl::animal as animal_schema;
#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/users", post(create_user))
        .route("/animals", get(list_animals).post(create_animal))
        .route("/animals/:animal_id", get(get_animal));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

#[axum::debug_handler]
async fn list_animals() -> Result<impl IntoResponse, StatusCode> {
    let connection = &mut establish_connection();
    let animals = animal_schema
        .load::<Animal>(connection)
        .expect("couldn't load animals");
    Ok(Json(animals))
    // (StatusCode::OK, Json(animals))
}

#[axum::debug_handler]
async fn get_animal(Path(animal_id): Path<i32>) -> Result<impl IntoResponse, StatusCode> {
    match Animal::get_animal(animal_id).await {
        Ok(animal) => Ok(Json(animal)),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
    // Ok(Json(animal))
    // (StatusCode::OK, Json(animals))
}
#[axum::debug_handler]
async fn create_animal(Json(payload): Json<NewAnimal>) -> Result<impl IntoResponse, StatusCode> {
    if payload.name.is_empty(){
        return Err(StatusCode::BAD_REQUEST);
    }
    match Animal::create(payload).await {
        Ok(animal) => Ok(Json(animal)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}