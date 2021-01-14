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
                        <div class="title"><h2>{"Screenshots"}</h2></div>
                        <div class="desc gallery">
                            <div
                                class="img-holder"
                                style="
                                    background-image: url('/images/IMG_2445.jpg');
                                "
                            ></div>
                            <div
                                class="img-holder"
                                style="
                                    background-image: url('/images/IMG_2532_lower.jpg');
                                "
                            ></div>
                            <div
                                class="img-holder"
                                style="
                                    background-image: url('/images/pexels-mont-photographs-2948636.jpg');
                                "
                            ></div>
                            <div
                                class="img-holder"
                                style="
                                    background-image: url('/images/pexels-spacex-23793.jpg');
                                "
                            ></div>
                            <div
                                class="img-holder"
                                style="
                                    background-image: url('/images/photo_2020-10-22_16-23-09.jpg');
                                "
                            ></div>
                        </div>
                        <div class="title"><h2>{"Description"}</h2></div>
                        <div class="desc text">
                            <h3>{"November 2020 (version 1.52)"}</h3>

                            <p>{"Update 1.52.1: The update addresses these issues."}</p>
                            <p>
                                {"Downloads: Windows: User System ARM | Mac | Linux:
                                snap deb rpm tarball ARM"}
                            </p>

                            <br />
                            <p>
                                {"To the November 2020 release of Visual Studio Code.
                                As announced in the November iteration plan, we
                                continued to focus for two weeks on housekeeping
                                GitHub issues and pull requests as documented in our
                                issue grooming guide. Across all of our VS Code
                                repositories, we closed (either triaged or fixed)
                                5242 issues, which is even more than during our last
                                housekeeping iteration in October 2019, where we
                                closed 4622 issues. While we closed issues, you
                                created 2937 new issues. The main vscode repository
                                now has 2146 open feature requests and 884 open
                                bugs. In addition, we closed 144 pull requests."}
                            </p>
                        </div>
                        <div class="title"><h2>{"Tutorials"}</h2></div>
                        <div class="desc gallery">
                            <div
                                class="img-holder"
                                style="
                                    background-image: url('/images/IMG_2445.jpg');
                                "
                            ></div>
                            <div
                                class="img-holder"
                                style="
                                    background-image: url('/images/IMG_2532_lower.jpg');
                                "
                            ></div>
                            <div
                                class="img-holder"
                                style="
                                    background-image: url('/images/pexels-mont-photographs-2948636.jpg');
                                "
                            ></div>
                            <div
                                class="img-holder"
                                style="
                                    background-image: url('/images/pexels-spacex-23793.jpg');
                                "
                            ></div>
                            <div
                                class="img-holder"
                                style="
                                    background-image: url('/images/photo_2020-10-22_16-23-09.jpg');
                                "
                            ></div>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
