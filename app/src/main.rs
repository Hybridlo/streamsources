mod util;
mod state;
mod components;

use state::ClientConfig;
use yew::prelude::*;
use yew_router::prelude::*;
use components::*;
use yewdux::prelude::use_store;

use web_sys::console::log_1;

const ERROR_MODAL: &str = "errorModal";

#[function_component(App)]
fn app() -> Html {
    let (_, config_setter) = use_store::<ClientConfig>();

    config_setter.reduce_mut(|config| {
        let window = web_sys::window().unwrap();
        let location = window.location();
        let path = location.origin().unwrap();
        config.config.base_path = path;

        log_1(&"Reduce mut on ClientConfig".into());
    });

    html! {
        // wrap the whole app in router to have history context everywhere
        <BrowserRouter>
            <ErrorModal elem_id={ERROR_MODAL.to_string()} />
            <Base />
        </BrowserRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}