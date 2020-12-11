use yew::prelude::*;

use crate::components::{Footer, Header};
use crate::layouts::MainLayout;
pub struct Home;

impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <MainLayout/>
        }
    }
}
