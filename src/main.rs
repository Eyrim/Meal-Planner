use dioxus::prelude::*;

use crate::meal::Meal;

mod meal;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
    }
}

fn get_meals() -> Meal {
}

