use crate::components::SideNav;
use crate::components::{Footer, Header};
use crate::data::{cats, Category};
use crate::layouts::MainLayout;
use crate::routes::{AppRoute, AppRouteAnchor};
use yew::prelude::*;
pub struct Categories;

impl Component for Categories {
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
        let CATS = cats();
        html! {
            <div id="container">
                <SideNav/>
                <div id="main">

                    <div class="title"><h2>{"Categories"}</h2></div>
                    {
                        CATS.iter().map(|cat| {
                            html! {
                                <div class="cate-card" style={&format!("background-image: url('http://127.0.0.1:5500{}');", cat.icon.clone())}>
                                    <h3>{ cat.name.clone() }</h3>
                                </div>
                            }
                        }).collect::<Html>()
                    }


                    <div class="clear"></div>
                </div>
            </div>
        }
    }
}
