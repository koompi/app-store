use wasm_bindgen::prelude::*;
use yew::prelude::*;

pub struct Index;

impl Component for Index {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>{"Hi"}</div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Index>::new().mount_to_body();
}
