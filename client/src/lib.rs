#![allow(non_snake_case)]

// Modules
pub mod components;
pub mod layouts;
pub mod pages;
pub mod routes;
// Library imports
use wasm_bindgen::prelude::*;
use yew::{html, App, Component, ComponentLink, Html, ShouldRender};

// Local imports
use pages::Home;
pub struct Index;

impl Component for Index {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <Home/>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Index>::new().mount_to_body();
}
