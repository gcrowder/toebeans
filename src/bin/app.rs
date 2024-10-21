use dioxus::prelude::*;

fn main() {
    launch(app);
}

fn app() -> Element {
    rsx!{
        h1{
            onclick: |_| {
                println!("Hello World!");
            },
            "Toebeans! And more!",
        }
    }
}