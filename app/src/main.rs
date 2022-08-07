mod components;

use anyhow::Result;
use twitch_sources_rework::MyTestStruct;
use yew::prelude::*;
use yew_router::prelude::*;
use components::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        // wrap the whole app in router to have history context everywhere
        <BrowserRouter>
            <Base />
        </BrowserRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}