use crate::components::SideNav;
use crate::components::{Footer, Header};
use crate::layouts::MainLayout;
use yew::prelude::*;
pub struct Updates;

impl Component for Updates {
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
                <div id="main" class="update">
                    <div class="title"><h2>{"Updates available"}</h2></div>
                    <div class="list">
                        <div
                            class="avatar"
                            style="
                                background-image: url('https://cdn.worldvectorlogo.com/logos/visual-studio-code-1.svg');
                            "
                        ></div>

                        <div class="info">
                            <h3>{"Visual Studio Code"}</h3>
                            <p>
                                {"The best open source text editor. The best open
                                source text editor. The best open source text
                                editor."}
                            </p>
                        </div>
                        <div class="update">
                            <button>{"Update"}</button>
                        </div>
                    </div>
                    <div class="list">
                        <div
                            class="avatar"
                            style="
                                background-image: url('https://cdn.worldvectorlogo.com/logos/visual-studio-code-1.svg');
                            "
                        ></div>

                        <div class="info">
                            <h3>{"Visual Studio Code"}</h3>
                            <p>
                                {"The best open source text editor. The best open
                                source text editor. The best open source text
                                editor."}
                            </p>
                        </div>
                        <div class="update">
                            <button>{"Update"}</button>
                        </div>
                    </div>
                    <div class="list">
                        <div
                            class="avatar"
                            style="
                                background-image: url('https://cdn.worldvectorlogo.com/logos/visual-studio-code-1.svg');
                            "
                        ></div>

                        <div class="info">
                            <h3>{"Visual Studio Code"}</h3>
                            <p>
                                {"The best open source text editor. The best open
                                source text editor. The best open source text
                                editor."}
                            </p>
                        </div>
                        <div class="update">
                            <button>{"Update"}</button>
                        </div>
                    </div>
                    <div class="list">
                        <div
                            class="avatar"
                            style="
                                background-image: url('https://cdn.worldvectorlogo.com/logos/visual-studio-code-1.svg');
                            "
                        ></div>

                        <div class="info">
                            <h3>{"Visual Studio Code"}</h3>
                            <p>
                                {"The best open source text editor. The best open
                                source text editor. The best open source text
                                editor."}
                            </p>
                        </div>
                        <div class="update">
                            <button>{"Update"}</button>
                        </div>
                    </div>
                    <div class="list">
                        <div
                            class="avatar"
                            style="
                                background-image: url('https://cdn.worldvectorlogo.com/logos/visual-studio-code-1.svg');
                            "
                        ></div>

                        <div class="info">
                            <h3>{"Visual Studio Code"}</h3>
                            <p>
                                {"The best open source text editor. The best open
                                source text editor. The best open source text
                                editor."}
                            </p>
                        </div>
                        <div class="update">
                            <button>{"Update"}</button>
                        </div>
                    </div>
                    <div class="list">
                        <div
                            class="avatar"
                            style="
                                background-image: url('https://cdn.worldvectorlogo.com/logos/visual-studio-code-1.svg');
                            "
                        ></div>

                        <div class="info">
                            <h3>{"Visual Studio Code"}</h3>
                            <p>
                                {"The best open source text editor. The best open
                                source text editor. The best open source text
                                editor."}
                            </p>
                        </div>
                        <div class="update">
                            <button>{"Update"}</button>
                        </div>
                    </div>
                    <div class="list">
                        <div
                            class="avatar"
                            style="
                                background-image: url('https://cdn.worldvectorlogo.com/logos/visual-studio-code-1.svg');
                            "
                        ></div>

                        <div class="info">
                            <h3>{"Visual Studio Code"}</h3>
                            <p>
                                {"The best open source text editor. The best open
                                source text editor. The best open source text
                                editor."}
                            </p>
                        </div>
                        <div class="update">
                            <button>{"Update"}</button>
                        </div>
                    </div>
                    <div class="list">
                        <div
                            class="avatar"
                            style="
                                background-image: url('https://cdn.worldvectorlogo.com/logos/visual-studio-code-1.svg');
                            "
                        ></div>

                        <div class="info">
                            <h3>{"Visual Studio Code"}</h3>
                            <p>
                                {"The best open source text editor. The best open
                                source text editor. The best open source text
                                editor."}
                            </p>
                        </div>
                        <div class="update">
                            <button>{"Update"}</button>
                        </div>
                    </div>
                    <div class="list">
                        <div
                            class="avatar"
                            style="
                                background-image: url('https://cdn.worldvectorlogo.com/logos/visual-studio-code-1.svg');
                            "
                        ></div>

                        <div class="info">
                            <h3>{"Visual Studio Code"}</h3>
                            <p>
                                {"The best open source text editor. The best open
                                source text editor. The best open source text
                                editor."}
                            </p>
                        </div>
                        <div class="update">
                            <button>{"Update"}</button>
                        </div>
                    </div>
                    <div class="list">
                        <div
                            class="avatar"
                            style="
                                background-image: url('https://cdn.worldvectorlogo.com/logos/visual-studio-code-1.svg');
                            "
                        ></div>

                        <div class="info">
                            <h3>{"Visual Studio Code"}</h3>
                            <p>
                                {"The best open source text editor. The best open
                                source text editor. The best open source text
                                editor."}
                            </p>
                        </div>
                        <div class="update">
                            <button>{"Update"}</button>
                        </div>
                    </div>
                    <div class="list">
                        <div
                            class="avatar"
                            style="
                                background-image: url('https://cdn.worldvectorlogo.com/logos/visual-studio-code-1.svg');
                            "
                        ></div>

                        <div class="info">
                            <h3>{"Visual Studio Code"}</h3>
                            <p>
                                {"The best open source text editor. The best open
                                source text editor. The best open source text
                                editor."}
                            </p>
                        </div>
                        <div class="update">
                            <button>{"Update"}</button>
                        </div>
                    </div>
                    <div class="list">
                        <div
                            class="avatar"
                            style="
                                background-image: url('https://cdn.worldvectorlogo.com/logos/visual-studio-code-1.svg');
                            "
                        ></div>

                        <div class="info">
                            <h3>{"Visual Studio Code"}</h3>
                            <p>
                                {"The best open source text editor. The best open
                                source text editor. The best open source text
                                editor."}
                            </p>
                        </div>
                        <div class="update">
                            <button>{"Update"}</button>
                        </div>
                    </div>
                    <div class="list">
                        <div
                            class="avatar"
                            style="
                                background-image: url('https://upload.wikimedia.org/wikipedia/commons/thumb/8/80/Atom_editor_logo.svg/131px-Atom_editor_logo.svg.png');
                            "
                        ></div>

                        <div class="info">
                            <h3>{"Atom IDE"}</h3>
                            <p>
                                {"The best open source text editor. The best open
                                source text editor. The best open source text
                                editor."}
                            </p>
                        </div>
                        <div class="update">
                            <button>{"Update"}</button>
                        </div>
                    </div>
                    <div class="clear"></div>
                </div>
            </div>
        }
    }
}
