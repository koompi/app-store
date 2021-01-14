use crate::components::SideNav;
use crate::components::{Footer, Header};
use crate::layouts::MainLayout;
use crate::routes::{AppRoute, AppRouteAnchor};
use yew::prelude::*;
pub struct Assets;

impl Component for Assets {
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
                        <h1>{"Apps"}</h1>
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
                    <div class="title"><h2>{"Featured applications"}</h2></div>
                    <div class="card">
                        <AppRouteAnchor classes="home-logo-link" route={AppRoute::AppByNamePage(String::from("visual-studio-code"))}>
                            <div
                                class="img-container"
                                style="background-image: url('/images/IMG_2532_lower.jpg')"
                            ></div>
                            <div class="info">
                                <h3>{"Visual Studio Code"}</h3>
                                <p>
                                    {"The best open source text editor. The best open
                                    source text editor. The best open source text
                                    editor."}
                                </p>
                            </div>

                            <div
                                class="avatar"
                                style="
                                    background-image: url('https://cdn.worldvectorlogo.com/logos/visual-studio-code-1.svg');
                                "
                            ></div>
                        </AppRouteAnchor>
                    </div>
                    <div class="card">
                        <div
                            class="img-container"
                            style="background-image: url('/images/IMG_2532_lower.jpg')"
                        ></div>
                        <div class="info">
                            <h3>{"Atom IDE"}</h3>
                            <p>
                                {"The best open source text editor. The best open
                                source text editor. The best open source text
                                editor."}
                            </p>
                        </div>
                        <div
                            class="avatar"
                            style="
                                background-image: url('https://upload.wikimedia.org/wikipedia/commons/thumb/8/80/Atom_editor_logo.svg/131px-Atom_editor_logo.svg.png');
                            "
                        ></div>
                    </div>

                    <div class="clear"></div>
                </div>
            </div>
        }
    }
}
