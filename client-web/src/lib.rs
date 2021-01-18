#![recursion_limit = "1024"]
#![allow(non_snake_case, unused_variables, unused_imports)]

// Modules
pub mod components;
pub mod data;
pub mod graphql;
pub mod layouts;
pub mod pages;
pub mod routes;
// Library imports
use wasm_bindgen::prelude::*;
use yew::{html, App, Component, ComponentLink, Html, Properties, ShouldRender};

// Local imports
use components::SideNav;
use pages::Home;
use routes::{AppRoute, AppRouter};
#[derive(Debug, Default, Clone)]
pub struct Index {
    active_route: AppRoute,
}

impl Component for Index {
    type Message = ();
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            active_route: AppRoute::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <AppRouter/>
        }
    }
}
// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Index>::new().mount_to_body();
}
