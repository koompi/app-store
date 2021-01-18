use crate::components::SideNav;
use crate::components::{Footer, Header};
use crate::layouts::MainLayout;
use yew::prelude::*;

pub struct Add;

impl Component for Add {
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
                    <div id="banner">
                        <h1>{"Games"}</h1>
                        <p>
                            {"There where the abundant of freedom, knowledge, economy,
                            and trust shared"}
                        </p>
                        <div id="input-container">
                            <input
                                type="text"
                                name=""
                                id=""
                                placeholder="Explore applications"
                            />
                            <img src="/icons/search-black-18dp.svg" alt="" />
                        </div>
                    </div>


                    <div class="clear"></div>
                </div>
            </div>
        }
    }
}
