use crate::components::SideNav;
use crate::components::{Footer, Header};
use crate::layouts::MainLayout;
use yew::prelude::*;

pub struct Settings;

impl Component for Settings {
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
                    <div class="paper setting">
                        <div class="title">
                            <h2>{"Account"}</h2>
                        </div>
                        <div class="desc">
                            <label for="full-name">{"Full Name"}</label>
                            <input type="text" name="full-name" />

                            <label for="full-name">{"User Name"}</label>
                            <input type="text" name="full-name" />

                            <label for="full-name">{"Email"}</label>
                            <input type="text" name="full-name" />

                            <label for="">{"Password"}</label>
                            <button>{"Change password"}</button>

                            <label for="">{"Two-Factor Auth"}</label>
                            <button>{"Enable"}</button>
                        </div>
                        <div class="title"><h2>{"Updates"}</h2></div>
                        <div class="desc">
                            <label for="">{"Automatic Update"}</label>
                            <button>{"Enable"}</button>

                            <label for="">{"Automatic Update"}</label>
                            <button>{"Enable"}</button>

                            <label for="">{"Automatic Update"}</label>
                            <button>{"Enable"}</button>
                            <label for="">{"Automatic Update"}</label>
                            <button>{"Enable"}</button>
                            <label for="">{"Automatic Update"}</label>
                            <button>{"Enable"}</button>
                            <label for="">{"Automatic Update"}</label>
                            <button>{"Enable"}</button>
                        </div>

                        <div class="clear"></div>
                    </div>
                </div>
            </div>
        }
    }
}
