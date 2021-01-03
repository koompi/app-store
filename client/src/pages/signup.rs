use yew::prelude::*;

pub struct Signup;

impl Component for Signup {
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
                    <h2>{"Sign Up"}</h2>
                    <form action="">
                        <input
                            type="email"
                            name="email"
                            id="email"
                            placeholder="Email"
                        />
                        <input
                            type="password"
                            name="password"
                            id="password"
                            placeholder="Password"
                        />
                        <input
                            type="password"
                            name="password"
                            id="password"
                            placeholder="Confirm password"
                        />
                        <button>{"Sign up"}</button>
                    </form>
                </div>
            </div>
        }
    }
}
