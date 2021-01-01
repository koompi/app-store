use yew::prelude::*;

use crate::components::SideNav;
use crate::components::{Footer, Header};
use crate::layouts::MainLayout;
pub struct AppByName;

impl Component for AppByName {
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
            <div id="container">
                <SideNav/>
                <div id="main">
                    <div id="banner"></div>
                    <div class="paper">
                        <div
                            class="avatar"
                            style="
                                background-image: url('https://cdn.worldvectorlogo.com/logos/visual-studio-code-1.svg');
                            "
                        ></div>
                        <div class="info">
                            <h1>{"Visual Studio Code"}</h1>
                            <button>{"Install"}</button>
                        </div>
                        <div class="desc"></div>
                        <div class="desc"></div>
                        <div class="desc"></div>
                        <div class="clear"></div>
                    </div>
                </div>
            </div>
        }
    }
}
