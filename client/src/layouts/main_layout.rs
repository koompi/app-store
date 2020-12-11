use yew::prelude::*;

use crate::components::{Content, SideNav};

pub struct MainLayout;

impl Component for MainLayout {
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
            <div class="container">
                <SideNav/>
                <Content/>
            </div>
        }
    }
}
