use yew::prelude::*;

pub struct Login;

impl Component for Login {
    type Message = ();
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div id="container" class="form-page">
            <div id="form-container">
                <h2>{"Login"}</h2>
                <form action="">
                    <input
                        type="email"
                        name="email"
                        id="email"
                        placeholder="user@koompi.com"
                    />
                    <input
                        type="password"
                        name="password"
                        id="password"
                        placeholder="314159265359"
                    />
                    <button>{"Login"}</button>
                    <button>{"Sign up"}</button>
                </form>
            </div>
        </div>
        }
    }
}
