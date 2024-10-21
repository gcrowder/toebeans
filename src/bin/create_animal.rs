use toebeans::*;
use std::io::{stdin};

fn main() {
    let mut name = String::new();
    let mut species = String::new();
    let mut microchip = String::new();

    println!("What would you like your animal name to be?");
    stdin().read_line(&mut name).unwrap();
    let name = name.trim_end().to_string(); // Remove the trailing newline

    println!("\nOk! What species is {name}?");
    stdin().read_line(&mut species).unwrap();
    let species = species.trim_end().to_string();

    println!("\nDoes your animal have a microchip number? What is it??");
    stdin().read_line(&mut microchip).unwrap();
    let microchip = match microchip.trim_end().parse::<i32>(){
        Ok(microchip) => Some(microchip),
        Err(e) => {
            eprintln!("{e}");
            None
        }
    };

    let animal = create_animal(name.clone(), Some(species.clone()), microchip.clone());
    let mut microchip_string = String::new();
    if microchip.is_some() {
        microchip_string += format!(" with microchip {:?}", microchip.unwrap()).as_str();
    }
    println!("\nSaved animal {species} {name} with id {}{microchip_string}", animal.id);
}

