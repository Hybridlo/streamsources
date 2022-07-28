mod components;

use anyhow::Result;
use twitch_sources_rework::MyTestStruct;
use yew::prelude::*;
use yew_router::prelude::*;
use components::header::Header;

#[function_component(Model)]
fn model() -> Html {
    html! {
        <>
            <Header />
            <div>
                <button>{"+1"}</button>
                <button>{"+2"}</button>
                <button>{"Fetch"}</button>
                <button>{"Fetch"}</button>
            </div>
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Model>();
}