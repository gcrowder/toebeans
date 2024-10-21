use diesel::{QueryDsl, RunQueryDsl};
use dioxus::prelude::*;
use toebeans::{create_animal, establish_connection};
use toebeans::models::Animal;

fn main() {
    launch(app);
}

fn create_animal_from_form(name: String, species: Option<String>, microchip: Option<i32>) {
    create_animal(name, species, microchip);
}

fn get_animals() -> Vec<Animal> {
    use toebeans::schema::animal::dsl::{animal, id, name, species, microchip};
    let connection = &mut establish_connection();
    
    animal.select((id, name, species, microchip))
        .limit(5)
        .load(connection)
        .expect("Error loading animals")
}

struct State {
    animals: Vec<Animal>,
}

fn app() -> Element {
    use_context_provider(|| Signal::new(State { animals: get_animals() }));
    let mut count = use_signal(||0);
    let state = use_context::<Signal<State>>();

    rsx!{
        h1{
            "Toebeans!",
        }
        ul {
            for animal in state.read().animals.iter() {
                li {
                    "{animal.name}"
                }
            }
        }
        form {
            onsubmit: move |event| {
                let data = event.data.values();
                let name = data.get("name").unwrap().first().unwrap();
                let species = data.get("species").unwrap().first().unwrap();
                let microchip: i32 = data.get("microchip").unwrap().first().unwrap().parse::<i32>().expect("That's not an integer!");
                create_animal_from_form(name.clone(), Some(species.clone()), Some(microchip));
            },
            input {
                r#type: "text",
                required: true,
                placeholder: "Enter your pet's name",
            }
            input {
                r#type: "text",
                required: true,
                placeholder: "Enter your pet's species",
            }
            input {
                r#type: "number",
                placeholder: "Enter your pet's microchip number",
            }
            button {
                r#type: "submit",
                "Submit"
            }
        }
    }
}