mod state;
mod components;

use anyhow::Result;
use state::ClientConfig;
use twitch_sources_rework::MyTestStruct;
use twitch_sources_client::apis::configuration::Configuration;
use yew::prelude::*;
use yew_router::prelude::*;
use components::*;
use yewdux::prelude::use_store;

#[function_component(App)]
fn app() -> Html {
    let (_, config_setter) = use_store::<ClientConfig>();

    config_setter.reduce_mut(|config| {
        let window = web_sys::window().unwrap();
        let location = window.location();
        let path = location.origin().unwrap();
        config.config.base_path = path;
    });

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